pub struct XorShift {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

impl XorShift {
    pub fn new(seed: u64) -> Self {
        let mut s = seed;
        let mut entity = XorShift { x: 123456789, y: 362436069, z: 521288629, w: 88675123 };
        s = (s ^ (s >> 30) + 0).wrapping_mul(1812433253);
        entity.x = s;
        s = (s ^ (s >> 30) + 1).wrapping_mul(1812433253);
        entity.y = s;
        s = (s ^ (s >> 30) + 2).wrapping_mul(1812433253);
        entity.z = s;
        s = (s ^ (s >> 30) + 3).wrapping_mul(1812433253);
        entity.w = s;
        entity
    }

    pub fn next(&mut self) -> u64 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        self.w
    }

    pub fn next_normalize(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }

    pub fn next_bounded(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next_normalize()
    }
}