//! Test grid: two floors, walls, landside/airside zones, security lanes,
//! one-way exit passages, stairs and floor markings (cell cost factors).

/// Cell flag: not walkable (wall or void).
pub const WALL: u8 = 1;
/// Cell flag: stairs/escalator connecting both floors at this (x, y).
pub const STAIRS: u8 = 2;

/// Zones. Allowed transitions: same zone, landside→security→airside,
/// airside→exit→landside (one-way passages as at a real airport).
pub const LANDSIDE: u8 = 0;
pub const AIRSIDE: u8 = 1;
pub const SECURITY: u8 = 2;
pub const EXIT: u8 = 3;

/// Default cost factor in tenths (10 = normal floor).
pub const COST_NORMAL: u8 = 10;
/// Floor marking "preferred route".
pub const COST_MARKED: u8 = 7;
/// Floor marking "avoid".
pub const COST_AVOID: u8 = 20;

/// Whether a person may step from a cell in zone `from` into zone `to`.
#[must_use]
pub fn zone_step_allowed(from: u8, to: u8) -> bool {
    from == to
        || matches!(
            (from, to),
            (LANDSIDE, SECURITY) | (SECURITY, AIRSIDE) | (AIRSIDE, EXIT) | (EXIT, LANDSIDE)
        )
}

/// Kinds of targets; one direction field per target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetKind {
    Gate(u8),
    CheckIn(u8),
    ToiletLandside,
    ToiletAirside,
    ShopAirside,
    ExitLandside,
}

#[derive(Clone)]
pub struct Target {
    pub kind: TargetKind,
    /// Seed cells (all at distance 0).
    pub cells: Vec<u32>,
}

#[derive(Clone)]
pub struct Grid {
    pub w: usize,
    pub h: usize,
    pub flags: Vec<u8>,
    pub zone: Vec<u8>,
    pub cost: Vec<u8>,
    pub targets: Vec<Target>,
    /// Landside spawn cells (entrance row).
    pub entrances: Vec<u32>,
    /// Cells of the security lanes (for build commands).
    pub security_cells: Vec<u32>,
    /// Door cells in interior walls (for build commands).
    pub doors: Vec<u32>,
}

/// Number of check-in groups (airlines) in the test layout.
pub const AIRLINES: u8 = 8;

impl Grid {
    #[must_use]
    pub fn cells(&self) -> usize {
        2 * self.w * self.h
    }

    #[must_use]
    pub fn idx(&self, x: usize, y: usize, f: usize) -> usize {
        (f * self.h + y) * self.w + x
    }

    #[must_use]
    pub fn xyf(&self, i: usize) -> (usize, usize, usize) {
        let per = self.w * self.h;
        (i % self.w, (i % per) / self.w, i / per)
    }

    #[must_use]
    pub fn walkable(&self, i: usize) -> bool {
        self.flags[i] & WALL == 0
    }

    /// Procedural layout in the spirit of the reference scenario: landside hall
    /// with check-in (floor 0, south), security line with lanes, airside on two
    /// floors connected by stairs, gates on floor 1, one-way exit passages.
    #[must_use]
    pub fn reference_like(w: usize, h: usize, gates: u8) -> Self {
        let n = 2 * w * h;
        let mut g = Self {
            w,
            h,
            flags: vec![0; n],
            zone: vec![LANDSIDE; n],
            cost: vec![COST_NORMAL; n],
            targets: Vec::new(),
            entrances: Vec::new(),
            security_cells: Vec::new(),
            doors: Vec::new(),
        };
        let yb = h / 2;
        for f in 0..2 {
            for y in 0..h {
                for x in 0..w {
                    let i = g.idx(x, y, f);
                    if y < yb {
                        g.zone[i] = AIRSIDE;
                    }
                    let border = x == 0 || y == 0 || x == w - 1 || y == h - 1;
                    // Floor 1 exists only above the airside area.
                    let void = f == 1 && y >= yb;
                    // Interior walls every 64 cells (offset 32), doors every 32 cells.
                    let vwall = x % 64 == 32 && y % 32 >= 3;
                    let hwall = y % 64 == 32 && y != yb && x % 32 >= 3;
                    let boundary = f == 0 && y == yb;
                    if border || void || vwall || hwall || boundary {
                        g.flags[i] |= WALL;
                    } else if (x % 64 == 32 && y % 32 < 3) || (y % 64 == 32 && x % 32 < 3) {
                        g.doors.push(i as u32);
                    }
                }
            }
        }
        // Security lanes (6 × 2 cells) and exit passages (2 × 3 cells) in the boundary row.
        for k in 0..6 {
            let x0 = w / 8 + k * (w * 3 / 4) / 6 + 6;
            for x in x0..x0 + 2 {
                let i = g.idx(x, yb, 0);
                g.flags[i] &= !WALL;
                g.zone[i] = SECURITY;
                g.security_cells.push(i as u32);
            }
        }
        for x0 in [w / 16, w - w / 16 - 3] {
            for x in x0..x0 + 3 {
                let i = g.idx(x, yb, 0);
                g.flags[i] &= !WALL;
                g.zone[i] = EXIT;
            }
        }
        // Stairs: 8 positions in airside, both floors.
        for k in 0..8 {
            let x = w * (2 * k + 1) / 16;
            let y = yb - 20;
            for f in 0..2 {
                let i = g.idx(x, y, f);
                g.flags[i] = STAIRS;
            }
        }
        // Floor markings: preferred corridor in front of the security line,
        // an "avoid" area in the landside hall.
        for y in yb + 1..yb + 12 {
            for x in w / 8..w - w / 8 {
                let i = g.idx(x, y, 0);
                g.cost[i] = COST_MARKED;
            }
        }
        for y in h * 3 / 4..h * 3 / 4 + 20 {
            for x in w / 3..w / 2 {
                let i = g.idx(x, y, 0);
                g.cost[i] = COST_AVOID;
            }
        }
        let free = |g: &Self, x: usize, y: usize, f: usize| {
            let i = g.idx(x, y, f);
            let mut c = Vec::new();
            for dy in 0..3 {
                for dx in 0..3 {
                    let j = g.idx(x + dx, y + dy, f);
                    if g.walkable(j) {
                        c.push(j as u32);
                    }
                }
            }
            if c.is_empty() {
                c.push(i as u32);
            }
            c
        };
        for k in 0..gates {
            let x = 4 + usize::from(k) * (w - 12) / usize::from(gates.max(1));
            let cells = free(&g, x, 4, 1);
            g.targets.push(Target {
                kind: TargetKind::Gate(k),
                cells,
            });
        }
        for a in 0..AIRLINES {
            let x = w / 8 + usize::from(a) * (w * 3 / 4) / usize::from(AIRLINES);
            let cells = free(&g, x, h - 60, 0);
            g.targets.push(Target {
                kind: TargetKind::CheckIn(a),
                cells,
            });
        }
        let multi = |g: &Self, pts: &[(usize, usize, usize)]| -> Vec<u32> {
            pts.iter().flat_map(|&(x, y, f)| free(g, x, y, f)).collect()
        };
        let tl = multi(
            &g,
            &[
                (w / 5, h - 30, 0),
                (w * 2 / 5, h - 30, 0),
                (w * 3 / 5, h - 30, 0),
                (w * 4 / 5, h - 30, 0),
            ],
        );
        let ta = multi(
            &g,
            &[
                (w / 5, yb - 40, 0),
                (w * 4 / 5, yb - 40, 0),
                (w / 5, 40, 1),
                (w * 4 / 5, 40, 1),
            ],
        );
        let sa = multi(
            &g,
            &[
                (w / 4, yb - 60, 0),
                (w / 2, yb - 60, 0),
                (w * 3 / 4, yb - 60, 0),
                (w / 6, 60, 1),
                (w / 3, 60, 1),
                (w / 2, 60, 1),
                (w * 2 / 3, 60, 1),
                (w * 5 / 6, 60, 1),
                (w * 5 / 6 - 20, 90, 1),
            ],
        );
        g.targets.push(Target {
            kind: TargetKind::ToiletLandside,
            cells: tl,
        });
        g.targets.push(Target {
            kind: TargetKind::ToiletAirside,
            cells: ta,
        });
        g.targets.push(Target {
            kind: TargetKind::ShopAirside,
            cells: sa,
        });
        let exit: Vec<u32> = (1..w - 1).map(|x| g.idx(x, h - 2, 0) as u32).collect();
        g.entrances.clone_from(&exit);
        g.targets.push(Target {
            kind: TargetKind::ExitLandside,
            cells: exit,
        });
        g
    }

    /// Index of the first target of the given kind.
    #[must_use]
    pub fn target_index(&self, kind: TargetKind) -> u8 {
        self.targets
            .iter()
            .position(|t| t.kind == kind)
            .expect("target") as u8
    }
}
