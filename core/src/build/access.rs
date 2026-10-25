//! Access profiles (ADR-037): which cells a group of persons may walk on.
//!
//! Every target carries the profile of the persons that walk to it; its flow
//! field is computed with the rules of that profile. The rules are a fixed
//! table "profile × area kind → walkable" plus two rules about steps: the
//! staff zone may only be left by persons without access, and the zone rule
//! (security in, exit out) applies to all but security staff and
//! construction workers.

use serde::{Deserialize, Serialize};

use super::grid::{AreaKind, KIND_MASK, KIND_SHIFT, STAFF_ZONE, WALL, Zone};

/// A group of persons with the same access to the cells of the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessProfile {
    /// Passengers and visitors: public walkways, apron walkways and
    /// crossings; staff zones only to leave.
    Passenger,
    /// Staff and franchise staff: like passengers, plus staff zones.
    Staff,
    /// Security staff: like staff, without the zone rule.
    Security,
    /// Apron staff: like staff, plus aircraft stand areas.
    ApronStaff,
    /// Construction workers: everywhere except runways in operation,
    /// without the zone rule.
    Worker,
}

impl AccessProfile {
    /// All profiles in the order of their numbers.
    pub const ALL: [Self; 5] = [
        Self::Passenger,
        Self::Staff,
        Self::Security,
        Self::ApronStaff,
        Self::Worker,
    ];

    /// The rules of the profile.
    #[must_use]
    pub fn rules(self) -> Rules {
        const WALKWAYS: u8 =
            bit(AreaKind::PublicWalk) | bit(AreaKind::ApronWalkway) | bit(AreaKind::Crossing);
        match self {
            Self::Passenger => Rules {
                kinds: WALKWAYS,
                enter_staff_zone: false,
                zone_rule: true,
            },
            Self::Staff => Rules {
                kinds: WALKWAYS,
                enter_staff_zone: true,
                zone_rule: true,
            },
            Self::Security => Rules {
                kinds: WALKWAYS,
                enter_staff_zone: true,
                zone_rule: false,
            },
            Self::ApronStaff => Rules {
                kinds: WALKWAYS | bit(AreaKind::StandArea),
                enter_staff_zone: true,
                zone_rule: true,
            },
            Self::Worker => Rules {
                // TODO(fahrplan-ref: 3.4): a runway is walkable for workers
                // while it is closed (RWY-03); until the runways have an
                // operating state they count as in operation.
                kinds: !bit(AreaKind::Runway),
                enter_staff_zone: true,
                zone_rule: false,
            },
        }
    }
}

const fn bit(kind: AreaKind) -> u8 {
    1 << kind as u8
}

/// The rules of one [`AccessProfile`], cheap to copy into the inner loop of
/// the flow field computation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rules {
    /// Bit `n` set: area kind `n` may be walked on.
    kinds: u8,
    /// A staff zone may be entered, not only left.
    enter_staff_zone: bool,
    /// Steps between zones follow [`Zone::step_allowed`].
    zone_rule: bool,
}

impl Rules {
    /// `true` if a person with these rules may stand on a cell with `flags`.
    #[must_use]
    pub fn walkable(self, flags: u8) -> bool {
        flags & WALL == 0 && (self.kinds >> ((flags & KIND_MASK) >> KIND_SHIFT)) & 1 != 0
    }

    /// `true` if a person with these rules may step from the cell with
    /// `from_flags` and `from_zone` to the neighbouring cell with
    /// `to_flags` and `to_zone`.
    #[must_use]
    pub fn step_allowed(self, from_flags: u8, from_zone: u8, to_flags: u8, to_zone: u8) -> bool {
        self.walkable(from_flags)
            && self.walkable(to_flags)
            && (self.enter_staff_zone || to_flags & STAFF_ZONE == 0 || from_flags & STAFF_ZONE != 0)
            && (!self.zone_rule || Zone::step_allowed(from_zone, to_zone))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::grid::Cell;

    fn flags(kind: AreaKind, staff: bool) -> u8 {
        Cell::floor(Zone::Landside)
            .with_kind(kind)
            .with_staff_zone(staff)
            .flags
    }

    fn walkable_kinds(profile: AccessProfile) -> Vec<AreaKind> {
        AreaKind::ALL
            .into_iter()
            .filter(|&k| profile.rules().walkable(flags(k, false)))
            .collect()
    }

    #[test]
    fn the_table_of_profiles_and_area_kinds() {
        use AreaKind::*;
        let walkways = vec![PublicWalk, ApronWalkway, Crossing];
        assert_eq!(walkable_kinds(AccessProfile::Passenger), walkways);
        assert_eq!(walkable_kinds(AccessProfile::Staff), walkways);
        assert_eq!(walkable_kinds(AccessProfile::Security), walkways);
        assert_eq!(
            walkable_kinds(AccessProfile::ApronStaff),
            vec![PublicWalk, ApronWalkway, Crossing, StandArea]
        );
        assert_eq!(
            walkable_kinds(AccessProfile::Worker),
            vec![
                PublicWalk,
                ApronWalkway,
                Crossing,
                StandArea,
                Apron,
                Road,
                Terrain
            ]
        );
    }

    #[test]
    fn walls_are_never_walkable() {
        for profile in AccessProfile::ALL {
            assert!(!profile.rules().walkable(WALL));
            assert!(!profile.rules().walkable(WALL | STAFF_ZONE));
        }
    }

    #[test]
    fn a_staff_zone_can_only_be_left_without_access() {
        let public = flags(AreaKind::PublicWalk, false);
        let staff = flags(AreaKind::PublicWalk, true);
        let l = Zone::Landside as u8;
        let passenger = AccessProfile::Passenger.rules();
        assert!(!passenger.step_allowed(public, l, staff, l), "no entry");
        assert!(passenger.step_allowed(staff, l, public, l), "leaving");
        assert!(passenger.step_allowed(staff, l, staff, l), "inside");
        for profile in [
            AccessProfile::Staff,
            AccessProfile::Security,
            AccessProfile::ApronStaff,
            AccessProfile::Worker,
        ] {
            assert!(profile.rules().step_allowed(public, l, staff, l));
        }
    }

    #[test]
    fn the_zone_rule_binds_all_but_security_and_workers() {
        let floor = flags(AreaKind::PublicWalk, false);
        let (landside, airside) = (Zone::Landside as u8, Zone::Airside as u8);
        for (profile, passes) in [
            (AccessProfile::Passenger, false),
            (AccessProfile::Staff, false),
            (AccessProfile::ApronStaff, false),
            (AccessProfile::Security, true),
            (AccessProfile::Worker, true),
        ] {
            assert_eq!(
                profile
                    .rules()
                    .step_allowed(floor, landside, floor, airside),
                passes,
                "{profile:?} from landside straight into airside"
            );
        }
    }
}
