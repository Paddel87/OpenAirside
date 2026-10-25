//! Catalog of the loaded content: what the simulation can build.

use std::collections::BTreeMap;

use super::model::{Effect, Placement};

/// An object type from a loaded pack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectType {
    /// Full id `<pack>:<entry>`.
    pub id: String,
    /// Localisation key of the name.
    pub name: String,
    /// How the object is placed.
    pub placement: Placement,
    /// What an `area` object does to the cells (ADR-042); `None` for a
    /// `single` object.
    pub effect: Option<Effect>,
    /// Whether a `single` object keeps persons from walking over its cells
    /// (ADR-042); `false` for an `area` object.
    pub blocks: bool,
}

/// A pack whose content is in the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivePack {
    /// Pack id from the manifest.
    pub id: String,
    /// Version of the pack (Semantic Versioning).
    pub version: String,
    /// Version of the pack format.
    pub format: u32,
}

/// Content of all active packs. Sorted maps keep every iteration
/// deterministic.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Catalog {
    objects: BTreeMap<String, ObjectType>,
    /// Active packs in load order (S-5: named in the save game metadata).
    packs: Vec<ActivePack>,
    /// Replaced full id → full id of the replacing entry.
    replaced: BTreeMap<String, String>,
    /// Full ids in the fixed order (set by [`Catalog::finish`]); the
    /// position is the index S-1 and the cells of the grid refer to.
    order: Vec<String>,
    base_failed: bool,
}

impl Catalog {
    /// Empty catalog (no pack loaded).
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// The object type with the full id `id`; a replaced entry resolves to
    /// the entry that replaces it.
    #[must_use]
    pub fn object(&self, id: &str) -> Option<&ObjectType> {
        let mut id = id;
        // Every replacement points to a pack loaded later, so chains are
        // finite; the bound only guards against a broken catalog.
        for _ in 0..=self.replaced.len() {
            match self.replaced.get(id) {
                Some(next) => id = next,
                None => return self.objects.get(id),
            }
        }
        None
    }

    /// Number of object types, replaced ones included.
    #[must_use]
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Full ids of all object types, sorted.
    pub fn object_ids(&self) -> impl Iterator<Item = &str> {
        self.objects.keys().map(String::as_str)
    }

    /// All object types, replaced ones included, in the order of their full
    /// ids. The order is fixed for the session: S-7 `catalog` lists the
    /// entries in it and S-1 refers to an entry by its index.
    pub fn objects(&self) -> impl Iterator<Item = &ObjectType> {
        self.objects.values()
    }

    /// Index of the entry with the full id `id` in the fixed order of
    /// [`Catalog::objects`], replaced entries included (no resolving of
    /// replacements: a cell shows what was built).
    #[must_use]
    pub fn index_of(&self, id: &str) -> Option<u32> {
        self.order
            .binary_search_by(|probe| probe.as_str().cmp(id))
            .ok()
            .and_then(|i| u32::try_from(i).ok())
    }

    /// Full id of the entry at `index` in the fixed order.
    #[must_use]
    pub fn id_at(&self, index: u32) -> Option<&str> {
        self.order
            .get(usize::try_from(index).ok()?)
            .map(String::as_str)
    }

    /// The entry that replaces `id` directly, if any.
    #[must_use]
    pub fn replaced_by(&self, id: &str) -> Option<&str> {
        self.replaced.get(id).map(String::as_str)
    }

    /// Active packs in load order.
    #[must_use]
    pub fn packs(&self) -> &[ActivePack] {
        &self.packs
    }

    /// The base pack of the game could not be loaded.
    #[must_use]
    pub fn base_failed(&self) -> bool {
        self.base_failed
    }

    /// Whether `id` exists (replaced or not).
    pub(super) fn contains(&self, id: &str) -> bool {
        self.objects.contains_key(id)
    }

    /// The pack entry that already replaces `id`.
    pub(super) fn replacement_of(&self, id: &str) -> Option<&str> {
        self.replaced.get(id).map(String::as_str)
    }

    pub(super) fn insert(&mut self, object: ObjectType) {
        self.objects.insert(object.id.clone(), object);
    }

    /// Fixes the order of the entries; the loader calls it once after the
    /// last pack.
    pub(super) fn finish(&mut self) {
        self.order = self.objects.keys().cloned().collect();
    }

    pub(super) fn replace(&mut self, target: String, by: String) {
        self.replaced.insert(target, by);
    }

    pub(super) fn add_pack(&mut self, pack: ActivePack) {
        self.packs.push(pack);
    }

    pub(super) fn set_base_failed(&mut self, failed: bool) {
        self.base_failed = failed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn object(id: &str) -> ObjectType {
        ObjectType {
            id: id.into(),
            name: format!("object.{id}"),
            placement: Placement::Area {},
            effect: Some(Effect::Wall {}),
            blocks: false,
        }
    }

    #[test]
    fn replaced_entries_resolve_along_the_chain() {
        let mut catalog = Catalog::empty();
        for id in ["base:a", "one:b", "two:c"] {
            catalog.insert(object(id));
        }
        catalog.replace("base:a".into(), "one:b".into());
        catalog.replace("one:b".into(), "two:c".into());
        assert_eq!(
            catalog.object("base:a").map(|o| o.id.as_str()),
            Some("two:c")
        );
        assert_eq!(
            catalog.object("two:c").map(|o| o.id.as_str()),
            Some("two:c")
        );
        assert!(catalog.object("base:x").is_none());
        assert_eq!(catalog.index_of("one:b"), None, "order is fixed by finish");
        catalog.finish();
        assert_eq!(catalog.index_of("base:a"), Some(0));
        assert_eq!(catalog.index_of("two:c"), Some(2));
        assert_eq!(catalog.index_of("base:x"), None);
        assert_eq!(catalog.id_at(1), Some("one:b"));
        assert_eq!(catalog.id_at(3), None);
        assert_eq!(catalog.object_count(), 3);
        assert_eq!(
            catalog.object_ids().collect::<Vec<_>>(),
            ["base:a", "one:b", "two:c"]
        );
    }

    #[test]
    fn a_broken_cycle_ends() {
        let mut catalog = Catalog::empty();
        catalog.replace("a:x".into(), "b:x".into());
        catalog.replace("b:x".into(), "a:x".into());
        assert!(catalog.object("a:x").is_none());
    }
}
