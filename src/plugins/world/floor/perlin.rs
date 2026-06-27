//! A from-scratch implementation of Ken Perlin's classic "Improved Noise"
//! (2002), generalized with a seedable permutation table. No external noise
//! crates involved — just the permutation/gradient/fade/lerp machinery.
//!
//! The permutation table is shuffled using whichever PRNG you choose from
//! `crate::util::rng` — swap generators to see how each one affects the terrain.

use crate::util::rng::RngKind;

pub struct Perlin {
    /// Permutation table, duplicated (0..256 then repeated) so lookups
    /// never need to wrap manually.
    perm: [u8; 512],
}

impl Perlin {
    /// Build a new permutation table from a seed and a choice of PRNG.
    /// Same seed + same RNG kind -> same terrain, every time.
    pub fn new(seed: u64, rng_kind: RngKind) -> Self {
        let mut p: [u8; 256] = core::array::from_fn(|i| i as u8);
        let mut rng = rng_kind.build(seed);

        // Fisher-Yates shuffle driven by the chosen PRNG.
        for i in (1..256).rev() {
            let j = (rng.next_u64() % (i as u64 + 1)) as usize;
            p.swap(i, j);
        }

        let mut perm = [0u8; 512];
        for i in 0..512 {
            perm[i] = p[i & 255];
        }
        Self { perm }
    }

    fn fade(t: f32) -> f32 {
        // 6t^5 - 15t^4 + 10t^3 — Perlin's improved ease curve.
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(t: f32, a: f32, b: f32) -> f32 {
        a + t * (b - a)
    }

    /// Maps the low 3 bits of a hash to one of 8 gradient directions in 2D.
    fn grad(hash: u8, x: f32, y: f32) -> f32 {
        match hash & 7 {
            0 => x + y,
            1 => -x + y,
            2 => x - y,
            3 => -x - y,
            4 => x,
            5 => -x,
            6 => y,
            _ => -y,
        }
    }

    /// Classic 2D Perlin noise. Output is roughly in [-1.0, 1.0].
    pub fn noise2d(&self, x: f32, y: f32) -> f32 {
        let xi = (x.floor() as i32 as usize) & 255;
        let yi = (y.floor() as i32 as usize) & 255;

        let xf = x - x.floor();
        let yf = y - y.floor();

        let u = Self::fade(xf);
        let v = Self::fade(yf);

        let a = self.perm[xi] as usize + yi;
        let aa = self.perm[a] as usize;
        let ab = self.perm[a + 1] as usize;
        let b = self.perm[xi + 1] as usize + yi;
        let ba = self.perm[b] as usize;
        let bb = self.perm[b + 1] as usize;

        let x1 = Self::lerp(
            u,
            Self::grad(self.perm[aa], xf, yf),
            Self::grad(self.perm[ba], xf - 1.0, yf),
        );
        let x2 = Self::lerp(
            u,
            Self::grad(self.perm[ab], xf, yf - 1.0),
            Self::grad(self.perm[bb], xf - 1.0, yf - 1.0),
        );

        Self::lerp(v, x1, x2)
    }

    /// Fractal Brownian Motion: stacks several octaves of noise at
    /// increasing frequency and decreasing amplitude, which is what turns
    /// flat Perlin noise into natural-looking terrain (big rolling hills
    /// plus small bumps on top). Output stays roughly in [-1.0, 1.0].
    pub fn fbm2d(&self, x: f32, y: f32, octaves: u32, persistence: f32, lacunarity: f32) -> f32 {
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let mut sum = 0.0;
        let mut max_amplitude = 0.0;

        for _ in 0..octaves {
            sum += self.noise2d(x * frequency, y * frequency) * amplitude;
            max_amplitude += amplitude;
            amplitude *= persistence;
            frequency *= lacunarity;
        }

        sum / max_amplitude
    }
}
