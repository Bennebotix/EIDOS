use rand::Rng;

const MIN_ALPHA: u8 = 10;
const MAX_ALPHA: u8 = 255;
const DEFAULT_ALPHA: u8 = 128;
const INITIAL_ALPHA_MIN: u8 = 10;
const INITIAL_ALPHA_MAX: u8 = 200;

const POSITION_MUTATION_RANGE: f64 = 16.0;
const RADIUS_MUTATION_RANGE: f64 = 8.0;
const ANGLE_MUTATION_RANGE: f64 = 0.5;
const ALPHA_MUTATION_RANGE: f64 = 30.0;

#[derive(Clone, Copy, Debug)]
pub struct Ellipse {
    pub x: f64,
    pub y: f64,
    pub rx: f64,
    pub ry: f64,
    pub angle: f64,
    pub color: (u8, u8, u8, u8),
    pub alpha: u8,
}

impl Ellipse {
    pub fn new_random(w: u32, h: u32) -> Self {
        let mut rng = rand::thread_rng();
        Ellipse {
            x: rng.gen_range(0.0..w as f64),
            y: rng.gen_range(0.0..h as f64),
            rx: rng.gen_range(1.0..32.0),
            ry: rng.gen_range(1.0..32.0),
            angle: rng.gen_range(0.0..std::f64::consts::PI),
            color: (0, 0, 0, DEFAULT_ALPHA),
            alpha: rng.gen_range(INITIAL_ALPHA_MIN..INITIAL_ALPHA_MAX),
        }
    }

    pub fn mutate(&mut self, w: u32, h: u32, iteration: usize, max_iter: usize) {
        let mut rng = rand::thread_rng();
        let progress = iteration as f64 / max_iter as f64;
        let scale = 1.0 - progress.powf(0.5);
        
        match rng.gen_range(0..6) {
            0 => self.x = (self.x + rng.gen_range(-POSITION_MUTATION_RANGE..POSITION_MUTATION_RANGE) * scale).clamp(0.0, w as f64),
            1 => self.y = (self.y + rng.gen_range(-POSITION_MUTATION_RANGE..POSITION_MUTATION_RANGE) * scale).clamp(0.0, h as f64),
            2 => self.rx = (self.rx + rng.gen_range(-RADIUS_MUTATION_RANGE..RADIUS_MUTATION_RANGE) * scale).clamp(0.5, w as f64),
            3 => self.ry = (self.ry + rng.gen_range(-RADIUS_MUTATION_RANGE..RADIUS_MUTATION_RANGE) * scale).clamp(0.5, h as f64),
            4 => self.angle = self.angle + rng.gen_range(-ANGLE_MUTATION_RANGE..ANGLE_MUTATION_RANGE) * scale,
            5 => {
                let delta = (rng.gen_range(-ALPHA_MUTATION_RANGE..ALPHA_MUTATION_RANGE) * scale) as i32;
                self.alpha = (self.alpha as i32 + delta).clamp(MIN_ALPHA as i32, MAX_ALPHA as i32) as u8;
            }
            _ => {},
        }
    }
}
