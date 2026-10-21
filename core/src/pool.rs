//! Storage of one kind of object in the world state (ADR-039): the records
//! of a kind lie densely in one vector (ADR-010), and the kind describes in
//! one place what a save game stores, how a loaded record is checked and
//! what it adds to the state hash.
//!
//! A domain module holds one [`Pool`] per kind of object; the world state
//! and the save game only know the modules, not the kinds.

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::hash::StateHasher;

/// A kind of object kept in a [`Pool`].
pub trait Kind: Copy + Serialize + DeserializeOwned {
    /// Most objects of this kind a world holds; a save game with more is
    /// rejected with `limitExceeded`.
    const MAX: usize;
    /// Name of the `limit` parameter when there are too many (S-5).
    const LIMIT: &'static str;
    /// Name of the `check` parameter of `invalidState` for a record that
    /// breaks an invariant (S-5).
    const CHECK: &'static str;
    /// What a loaded record is checked against: the parts of the world it
    /// refers to.
    type Context;

    /// `true` if a loaded record is valid in `context`.
    fn is_valid(&self, context: &Self::Context) -> bool;

    /// Feeds the record into the state hash.
    fn hash_into(&self, hasher: &mut StateHasher);
}

/// The objects of one kind, in storage order.
#[derive(Debug, Clone, PartialEq)]
pub struct Pool<T> {
    items: Vec<T>,
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: Kind> Pool<T> {
    /// Number of objects.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// `true` if there is no object.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Objects that still fit under [`Kind::MAX`].
    #[must_use]
    pub fn room(&self) -> usize {
        T::MAX.saturating_sub(self.items.len())
    }

    /// The objects in storage order.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    /// Adds an object; `false` (and nothing added) if the pool is full.
    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() >= T::MAX {
            return false;
        }
        self.items.push(item);
        true
    }

    /// Removes all objects.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// The vector of the objects, for a step of the owning module that
    /// rewrites all of them (e.g. by swapping in a second buffer). The
    /// module keeps the count within [`Kind::MAX`].
    pub(crate) fn items_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }

    /// The records as stored in a save game.
    #[must_use]
    pub fn saved(&self) -> Vec<T> {
        self.items.clone()
    }

    /// `true` if `count` loaded records fit under [`Kind::MAX`]; checked
    /// with the structure of a save game, before anything is rebuilt.
    #[must_use]
    pub fn fits(count: usize) -> bool {
        count <= T::MAX
    }

    /// Pool of a loaded save game.
    ///
    /// # Errors
    ///
    /// [`Kind::LIMIT`] if there are too many records, [`Kind::CHECK`] for
    /// the first record that is not valid in `context`.
    pub fn restore(saved: Vec<T>, context: &T::Context) -> Result<Self, &'static str> {
        if !Self::fits(saved.len()) {
            return Err(T::LIMIT);
        }
        if saved.iter().any(|item| !item.is_valid(context)) {
            return Err(T::CHECK);
        }
        Ok(Self { items: saved })
    }

    /// Feeds all records into the state hash, in storage order.
    pub fn hash_into(&self, hasher: &mut StateHasher) {
        for item in &self.items {
            item.hash_into(hasher);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    /// A kind of object that exists only here: everything it needs in the
    /// world state, the save game and the state hash is this one impl.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    struct Probe {
        cell: u16,
        charge: u8,
    }

    impl Kind for Probe {
        const MAX: usize = 3;
        const LIMIT: &'static str = "probes";
        const CHECK: &'static str = "probe";
        type Context = u16;

        fn is_valid(&self, cells: &u16) -> bool {
            self.cell < *cells && self.charge <= 100
        }

        fn hash_into(&self, hasher: &mut StateHasher) {
            hasher.u32(u32::from(self.cell));
            hasher.bytes(&[self.charge]);
        }
    }

    fn hash(pool: &Pool<Probe>) -> u64 {
        let mut h = StateHasher::new();
        pool.hash_into(&mut h);
        h.finish()
    }

    #[test]
    fn a_new_kind_is_stored_checked_and_hashed_by_its_own_impl() {
        let mut pool = Pool::<Probe>::default();
        assert!(pool.is_empty());
        for cell in 0..3 {
            assert!(pool.push(Probe { cell, charge: 50 }));
        }
        assert!(!pool.push(Probe { cell: 9, charge: 1 }), "full at MAX");
        assert_eq!((pool.len(), pool.room()), (3, 0));

        // Save and load through the format of the save game (postcard).
        let bytes = postcard::to_stdvec(&pool.saved()).unwrap();
        let loaded: Vec<Probe> = postcard::from_bytes(&bytes).unwrap();
        let restored = Pool::restore(loaded.clone(), &16).unwrap();
        assert_eq!(restored, pool);
        assert_eq!(hash(&restored), hash(&pool));

        // A record that breaks an invariant names the kind's check.
        let mut broken = loaded.clone();
        broken[1].charge = 101;
        assert_eq!(Pool::restore(broken, &16), Err("probe"));
        assert_eq!(Pool::restore(loaded.clone(), &2), Err("probe"));
        // Too many records name the kind's limit.
        let mut many = loaded;
        many.push(Probe { cell: 0, charge: 0 });
        assert!(!Pool::<Probe>::fits(many.len()));
        assert_eq!(Pool::restore(many, &16), Err("probes"));

        // Every record is part of the hash.
        let mut other = pool.clone();
        other.items_mut()[2].charge = 51;
        assert_ne!(hash(&other), hash(&pool));
        other.clear();
        assert_eq!(other.len(), 0);
        assert_eq!(other.as_slice(), &[]);
    }
}
