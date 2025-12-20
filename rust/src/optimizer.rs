use crate::shapes::Ellipse;
use rand::Rng;

const INITIAL_CANVAS_VALUE: u8 = 255;
const LATE_STAGE_THRESHOLD: f64 = 0.8;
const MID_STAGE_THRESHOLD: f64 = 0.5;
const LATE_STAGE_MAX_RADIUS: f64 = 10.0;
const MID_STAGE_MAX_RADIUS: f64 = 30.0;
const EARLY_STAGE_MAX_RADIUS: f64 = 200.0;

const STANDARD_MULTIPLIER: usize = 1;
const HIGH_FIDELITY_MULTIPLIER: usize = 3;
const SUPER_FIDELITY_MULTIPLIER: usize = 10;
const HYPER_FIDELITY_MULTIPLIER: usize = 100;

const BASE_RANDOM_TRIALS: usize = 40;
const BASE_HILL_CLIMB_STEPS: usize = 80;
const ERROR_SAMPLE_COUNT: usize = 30;
const INITIAL_SEED_MAX_RADIUS: f64 = 15.0;

const MIN_ALPHA_THRESHOLD: f64 = 0.01;

pub struct Optimizer {
    pub target_pixels: Vec<u8>,
    pub current_pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl Optimizer {
    pub fn new(target: &[u8], width: u32, height: u32) -> Self {
        let current = vec![INITIAL_CANVAS_VALUE; (width * height * 4) as usize];
        
        Optimizer {
            target_pixels: target.to_vec(),
            current_pixels: current,
            width,
            height,
        }
    }
    
    pub fn add_shape(&mut self, shape_idx: usize, max_shapes: usize, fidelity_mode: u8) -> Ellipse {
        let progress = shape_idx as f64 / max_shapes as f64;
        let max_radius = if progress > LATE_STAGE_THRESHOLD {
            LATE_STAGE_MAX_RADIUS
        } else if progress > MID_STAGE_THRESHOLD {
            MID_STAGE_MAX_RADIUS
        } else {
            EARLY_STAGE_MAX_RADIUS
        };

        let multiplier = match fidelity_mode {
            0 => STANDARD_MULTIPLIER,
            1 => HIGH_FIDELITY_MULTIPLIER,
            2 => SUPER_FIDELITY_MULTIPLIER,
            3 => HYPER_FIDELITY_MULTIPLIER,
            _ => STANDARD_MULTIPLIER,
        };
        
        let random_trials = BASE_RANDOM_TRIALS * multiplier;
        let hill_climb_steps = BASE_HILL_CLIMB_STEPS * multiplier;
        
        let mut best_shape = self.pick_high_error_seed(max_radius);
        let mut best_score = self.evaluate_shape(&best_shape);

        for _ in 0..random_trials {
            let shape = self.pick_high_error_seed(max_radius);
            let score = self.evaluate_shape(&shape);
            if score < best_score {
                best_score = score;
                best_shape = shape;
            }
        }
        
        let mut shape = best_shape;
        let mut score = best_score;
        
        for i in 0..hill_climb_steps {
            let mut new_shape = shape.clone();
            new_shape.mutate(self.width, self.height, i, hill_climb_steps);
            
            new_shape.rx = new_shape.rx.min(max_radius);
            new_shape.ry = new_shape.ry.min(max_radius);

            let new_score = self.evaluate_shape(&new_shape);
            if new_score < score {
                score = new_score;
                shape = new_shape;
            }
        }
        
        shape.color = self.compute_optimal_color(&shape);
        self.draw_shape(&shape);
        
        shape
    }

    fn pick_high_error_seed(&self, max_r: f64) -> Ellipse {
        let mut rng = rand::thread_rng();
        let mut best_x = rng.gen_range(0.0..self.width as f64);
        let mut best_y = rng.gen_range(0.0..self.height as f64);
        let mut max_error = -1.0;

        for _ in 0..ERROR_SAMPLE_COUNT {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            let idx = (y * self.width + x) as usize * 4;
            
            let tr = self.target_pixels[idx] as i32;
            let tg = self.target_pixels[idx+1] as i32;
            let tb = self.target_pixels[idx+2] as i32;
            
            let cr = self.current_pixels[idx] as i32;
            let cg = self.current_pixels[idx+1] as i32;
            let cb = self.current_pixels[idx+2] as i32;
            
            let error = ((tr-cr).pow(2) + (tg-cg).pow(2) + (tb-cb).pow(2)) as f64;
            
            if error > max_error {
                max_error = error;
                best_x = x as f64;
                best_y = y as f64;
            }
        }
        
        let mut s = Ellipse::new_random(self.width, self.height);
        s.x = best_x;
        s.y = best_y;
        s.rx = rng.gen_range(1.0..max_r.min(INITIAL_SEED_MAX_RADIUS)); 
        s.ry = rng.gen_range(1.0..max_r.min(INITIAL_SEED_MAX_RADIUS));
        s
    }

    fn evaluate_shape(&self, shape: &Ellipse) -> f64 {
        let r_max = shape.rx.max(shape.ry);
        let min_x = (shape.x - r_max).floor().max(0.0) as u32;
        let max_x = (shape.x + r_max).ceil().min(self.width as f64) as u32;
        let min_y = (shape.y - r_max).floor().max(0.0) as u32;
        let max_y = (shape.y + r_max).ceil().min(self.height as f64) as u32;
        
        if min_x >= max_x || min_y >= max_y { return f64::MAX; }

        let color = self.compute_optimal_color(shape);
        let (r, g, b, a) = color;
        let alpha_f = a as f64 / 255.0;

        let cos = shape.angle.cos();
        let sin = shape.angle.sin();
        let rx2 = shape.rx * shape.rx;
        let ry2 = shape.ry * shape.ry;

        let mut total_error_diff = 0i64;
        
        for y in min_y..max_y {
            for x in min_x..max_x {
                 let dx = x as f64 - shape.x;
                let dy = y as f64 - shape.y;
                let rot_x = dx * cos + dy * sin;
                let rot_y = -dx * sin + dy * cos;
                if (rot_x * rot_x) / rx2 + (rot_y * rot_y) / ry2 <= 1.0 {
                    let idx = (y * self.width + x) as usize * 4;
                    let tr = self.target_pixels[idx] as i32;
                    let tg = self.target_pixels[idx+1] as i32;
                    let tb = self.target_pixels[idx+2] as i32;
                    
                    let cr = self.current_pixels[idx] as i32;
                    let cg = self.current_pixels[idx+1] as i32;
                    let cb = self.current_pixels[idx+2] as i32;
                    
                    // Blend: New = Current*(1-a) + Shape*a
                    let nr = (cr as f64 * (1.0 - alpha_f) + r as f64 * alpha_f) as i32;
                    let ng = (cg as f64 * (1.0 - alpha_f) + g as f64 * alpha_f) as i32;
                    let nb = (cb as f64 * (1.0 - alpha_f) + b as f64 * alpha_f) as i32;
                    
                    let old_err = (tr-cr).pow(2) + (tg-cg).pow(2) + (tb-cb).pow(2);
                    let new_err = (tr-nr).pow(2) + (tg-ng).pow(2) + (tb-nb).pow(2);
                    
                    total_error_diff += new_err as i64 - old_err as i64;
                }
            }
        }
        total_error_diff as f64
    }
    
    fn compute_optimal_color(&self, shape: &Ellipse) -> (u8, u8, u8, u8) {
        let r_max = shape.rx.max(shape.ry);
        let min_x = (shape.x - r_max).floor().max(0.0) as u32;
        let max_x = (shape.x + r_max).ceil().min(self.width as f64) as u32;
        let min_y = (shape.y - r_max).floor().max(0.0) as u32;
        let max_y = (shape.y + r_max).ceil().min(self.height as f64) as u32;
        
        let mut sum_r = 0i64;
        let mut sum_g = 0i64;
        let mut sum_b = 0i64;
        let mut count = 0u64;
        let cos = shape.angle.cos();
        let sin = shape.angle.sin();
        let rx2 = shape.rx * shape.rx;
        let ry2 = shape.ry * shape.ry;
        
        let alpha = shape.alpha as f64 / 255.0;
        if alpha < MIN_ALPHA_THRESHOLD {
            return (0, 0, 0, 0);
        }

        for y in min_y..max_y {
            for x in min_x..max_x {
                let dx = x as f64 - shape.x;
                let dy = y as f64 - shape.y;
                let rot_x = dx * cos + dy * sin;
                let rot_y = -dx * sin + dy * cos;
                if (rot_x * rot_x) / rx2 + (rot_y * rot_y) / ry2 <= 1.0 {
                    let idx = (y * self.width + x) as usize * 4;
                    
                    let tr = self.target_pixels[idx] as f64;
                    let tg = self.target_pixels[idx+1] as f64;
                    let tb = self.target_pixels[idx+2] as f64;
                    
                    let cr = self.current_pixels[idx] as f64;
                    let cg = self.current_pixels[idx+1] as f64;
                    let cb = self.current_pixels[idx+2] as f64;
                    
                    let kr = (tr - cr * (1.0 - alpha)) / alpha;
                    let kg = (tg - cg * (1.0 - alpha)) / alpha;
                    let kb = (tb - cb * (1.0 - alpha)) / alpha;
                    
                    sum_r += kr.clamp(0.0, 255.0) as i64;
                    sum_g += kg.clamp(0.0, 255.0) as i64;
                    sum_b += kb.clamp(0.0, 255.0) as i64;
                    
                    count += 1;
                }
            }
        }
        
        if count == 0 {
            return (128, 128, 128, shape.alpha);
        }
        
        ((sum_r as u64 / count) as u8, 
         (sum_g as u64 / count) as u8, 
         (sum_b as u64 / count) as u8, 
         shape.alpha)
    }

    fn draw_shape(&mut self, shape: &Ellipse) {
        let r_max = shape.rx.max(shape.ry);
        let min_x = (shape.x - r_max).floor().max(0.0) as u32;
        let max_x = (shape.x + r_max).ceil().min(self.width as f64) as u32;
        let min_y = (shape.y - r_max).floor().max(0.0) as u32;
        let max_y = (shape.y + r_max).ceil().min(self.height as f64) as u32;
        
        let cos = shape.angle.cos();
        let sin = shape.angle.sin();
        let rx2 = shape.rx * shape.rx;
        let ry2 = shape.ry * shape.ry;
        let (r, g, b, a) = shape.color;
        let alpha_f = a as f64 / 255.0;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let dx = x as f64 - shape.x;
                let dy = y as f64 - shape.y;
                let rot_x = dx * cos + dy * sin;
                let rot_y = -dx * sin + dy * cos;
                if (rot_x * rot_x) / rx2 + (rot_y * rot_y) / ry2 <= 1.0 {
                    let idx = (y * self.width + x) as usize * 4;
                    let cr = self.current_pixels[idx] as f64;
                    let cg = self.current_pixels[idx+1] as f64;
                    let cb = self.current_pixels[idx+2] as f64;
                    
                    self.current_pixels[idx] = (cr * (1.0 - alpha_f) + r as f64 * alpha_f) as u8;
                    self.current_pixels[idx+1] = (cg * (1.0 - alpha_f) + g as f64 * alpha_f) as u8;
                    self.current_pixels[idx+2] = (cb * (1.0 - alpha_f) + b as f64 * alpha_f) as u8;
                    self.current_pixels[idx+3] = 255;
                }
            }
        }
    }
}
