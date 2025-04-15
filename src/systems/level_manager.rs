trait XPCurve {
    // returns the a
    fn xp_required(&self, level: u32) -> u32;
}

struct ExponentialCurve {
    // base xp required
    base: u32,
    exponent: f32,
}

impl ExponentialCurve {
    fn new(base: u32, exponent: f32) -> Self {
        Self { base, exponent }
    }
}

impl Default for ExponentialCurve {
    fn default() -> Self {
        Self {
            base: 2,
            exponent: 1.5,
        }
    }
}

impl XPCurve for ExponentialCurve {
    fn xp_required(&self, level: u32) -> u32 {
        // XP required = base * level^exponent, rounded up
        (self.base as f32 * (level as f32).powf(self.exponent)).ceil() as u32
    }
}

pub struct LevelManager {
    pub level: u32,
    pub current_xp: u32,
    pub xp_curve: Box<dyn XPCurve>,
}

impl LevelManager {
    pub fn new(initial_level: u32, xp_curve: Box<dyn XPCurve>) -> Self {
        Self {
            level: initial_level,
            current_xp: 0,
            xp_curve,
        }
    }

    pub fn add_xp(&mut self, xp: u32) -> u32 {
        let initial_level = self.level;

        self.current_xp += xp;
        // checks if there are any level ups
        while (self.current_xp >= self.xp_curve.xp_required(self.level + 1)) {
            self.current_xp -= self.xp_curve.xp_required(self.level + 1);
            self.level += 1;
        }

        self.level - initial_level
    }

    pub fn xp_to_next_level(&self) -> u32 {
        self.xp_curve.xp_required(self.level + 1)
    }
}

impl Default for LevelManager {
    fn default() -> Self {
        Self {
            level: 1,
            current_xp: 0,
            xp_curve: Box::new(ExponentialCurve::default()),
        }
    }
}
