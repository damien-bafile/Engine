//! A small zoo of pseudo-random number generators, all implemented from
//! scratch, so you can swap them in and out and see how each one affects
//! the permutation table (and therefore the terrain) Perlin noise is built
//! from. None of these are cryptographically secure — they're chosen for
//! speed, simplicity, and being easy to seed with a single `u64`.

/// Common interface every generator below implements.
pub trait Rng64 {
    /// Returns the next pseudo-random 64-bit value.
    fn next_u64(&mut self) -> u64;
}

/// SplitMix64 — Sebastiano Vigna's fast, simple generator. Often used just
/// to seed *other* generators because of its excellent avalanche behavior,
/// but it's a perfectly fine standalone generator too. This is what the
/// original version of this project used internally.
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Rng64 for SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// xorshift64* — George Marsaglia's xorshift core with a Vigna multiplier
/// applied to the output to fix its statistically weak low bits.
pub struct Xorshift64Star {
    state: u64,
}

impl Xorshift64Star {
    pub fn new(seed: u64) -> Self {
        // xorshift needs a non-zero seed or it gets stuck at 0 forever.
        Self {
            state: if seed == 0 { 0xDEAD_BEEF_CAFE_F00D } else { seed },
        }
    }
}

impl Rng64 for Xorshift64Star {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// xorshift128+ — a larger-state xorshift variant (as used in early V8)
/// with better statistical quality and a longer period than the plain
/// 64-bit version above.
pub struct Xorshift128Plus {
    s0: u64,
    s1: u64,
}

impl Xorshift128Plus {
    pub fn new(seed: u64) -> Self {
        // Spin up both halves of the state with SplitMix64 so a single u64
        // seed still produces well-distributed initial state.
        let mut seeder = SplitMix64::new(seed);
        let s0 = seeder.next_u64();
        let s1 = seeder.next_u64();
        Self {
            s0: if s0 == 0 { 1 } else { s0 },
            s1: if s1 == 0 { 2 } else { s1 },
        }
    }
}

impl Rng64 for Xorshift128Plus {
    fn next_u64(&mut self) -> u64 {
        let mut s1 = self.s0;
        let s0 = self.s1;
        self.s0 = s0;
        s1 ^= s1 << 23;
        s1 ^= s1 >> 17;
        s1 ^= s0;
        s1 ^= s0 >> 26;
        self.s1 = s1;
        self.s0.wrapping_add(self.s1)
    }
}

/// PCG32 (XSH-RR variant) — Melissa O'Neill's permuted congruential
/// generator. Small state, passes most statistical test suites, widely
/// used as a general-purpose default elsewhere.
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    pub fn new(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (stream << 1) | 1,
        };
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u32_raw();
        rng
    }

    fn next_u32_raw(&mut self) -> u32 {
        let old_state = self.state;
        self.state = old_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);
        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        (xorshifted >> rot) | (xorshifted << ((32u32.wrapping_sub(rot)) & 31))
    }
}

impl Rng64 for Pcg32 {
    fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32_raw() as u64;
        let lo = self.next_u32_raw() as u64;
        (hi << 32) | lo
    }
}

/// A classic linear congruential generator (using Knuth's MMIX constants).
/// Lower statistical quality than the others — useful as a "control group"
/// to see what a weaker generator does to the terrain by comparison.
pub struct Lcg64 {
    state: u64,
}

impl Lcg64 {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Rng64 for Lcg64 {
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
}

/// A wyhash-style mixer (inspired by Wang Yi's wyrand): very few
/// instructions per output, relying on a 128-bit multiply to scramble bits.
/// Generators like this are popular as fast default RNGs in some language
/// runtimes.
pub struct WyRand {
    state: u64,
}

impl WyRand {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl Rng64 for WyRand {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0xA076_1D64_78BD_642F);
        let t = (self.state as u128).wrapping_mul((self.state ^ 0xE703_7ED1_A0B4_28DB) as u128);
        ((t >> 64) ^ t) as u64
    }
}

/// Selects which generator to use. Flip this in `TerrainSettings` (or pass
/// a name on the command line, see `main.rs`) and re-run to compare.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RngKind {
    SplitMix64,
    Xorshift64Star,
    Xorshift128Plus,
    Pcg32,
    Lcg64,
    WyRand,
}

impl RngKind {
    /// Builds a boxed generator of this kind from a single `u64` seed.
    pub fn build(self, seed: u64) -> Box<dyn Rng64> {
        match self {
            RngKind::SplitMix64 => Box::new(SplitMix64::new(seed)),
            RngKind::Xorshift64Star => Box::new(Xorshift64Star::new(seed)),
            RngKind::Xorshift128Plus => Box::new(Xorshift128Plus::new(seed)),
            RngKind::Pcg32 => Box::new(Pcg32::new(seed, 0xDA3E_39CB_94B9_5BDB)),
            RngKind::Lcg64 => Box::new(Lcg64::new(seed)),
            RngKind::WyRand => Box::new(WyRand::new(seed)),
        }
    }

    /// Parses a generator name from a string, case-insensitively, accepting
    /// a couple of short aliases for convenience on the command line.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "splitmix64" | "splitmix" => Some(RngKind::SplitMix64),
            "xorshift64star" | "xorshift64" | "xorshift" => Some(RngKind::Xorshift64Star),
            "xorshift128plus" | "xorshift128" => Some(RngKind::Xorshift128Plus),
            "pcg32" | "pcg" => Some(RngKind::Pcg32),
            "lcg64" | "lcg" => Some(RngKind::Lcg64),
            "wyrand" | "wy" => Some(RngKind::WyRand),
            _ => None,
        }
    }

    /// All available names, mainly so `main.rs` can print a usage hint.
    pub const ALL_NAMES: &'static [&'static str] = &[
        "splitmix64",
        "xorshift64star",
        "xorshift128plus",
        "pcg32",
        "lcg64",
        "wyrand",
    ];

    pub fn name(&self) -> &'static str {
        match self {
            RngKind::SplitMix64 => "SplitMix64",
            RngKind::Xorshift64Star => "Xorshift64*",
            RngKind::Xorshift128Plus => "Xorshift128+",
            RngKind::Pcg32 => "PCG32",
            RngKind::Lcg64 => "LCG64",
            RngKind::WyRand => "WyRand",
        }
    }

    pub const ALL: [RngKind; 6] = [
        RngKind::SplitMix64,
        RngKind::Xorshift64Star,
        RngKind::Xorshift128Plus,
        RngKind::Pcg32,
        RngKind::Lcg64,
        RngKind::WyRand,
    ];
}
