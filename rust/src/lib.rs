mod image_ops;
mod color;
mod segmentation;
mod math;
mod desmos;
mod shapes;
mod optimizer;

use wasm_bindgen::prelude::*;
use desmos::{DesmosState, GraphSettings, Viewport, ExpressionList};

const MIN_SHAPE_ID: usize = 20;
const FOLDER_ID: &str = "8";
const AUTHOR_ID: &str = "2";
const INSTRUCTIONS_ID: &str = "4";
const GITHUB_ID: &str = "6";
const FOLDER_TITLE: &str = "Image";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log("Rust initiated!");
}

#[wasm_bindgen]
pub struct DesmosOptimizer {
    optimizer: optimizer::Optimizer,
    img_width: u32,
    img_height: u32,
    max_shapes: usize,
    current_shape_idx: usize,
    shapes: Vec<shapes::Ellipse>,
    fidelity_mode: u8,
}

#[wasm_bindgen]
impl DesmosOptimizer {
    #[wasm_bindgen(constructor)]
    pub fn new(image_data: &[u8], max_shapes: usize, fidelity_mode: u8) -> Result<DesmosOptimizer, JsValue> {
        let img_proc = image_ops::ImageProcessor::new(image_data)?;
        let width = img_proc.width();
        let height = img_proc.height();
        
        log(&format!("Optimizer initialized: {}x{}", width, height));

        let optimizer = optimizer::Optimizer::new(&img_proc.get_pixels(), width, height);
        
        Ok(DesmosOptimizer {
            optimizer,
            img_width: width,
            img_height: height,
            max_shapes,
            current_shape_idx: 0,
            shapes: Vec::new(),
            fidelity_mode,
        })
    }

    pub fn step(&mut self, batch_size: usize) -> bool {
        let start = self.current_shape_idx;
        let end = (start + batch_size).min(self.max_shapes);
        
        for i in start..end {
            if (i + 1) % 50 == 0 && i != self.max_shapes - 1 {
                log(&format!("Added shape {}/{}", i + 1, self.max_shapes));
            }
            
            let shape = self.optimizer.add_shape(i, self.max_shapes, self.fidelity_mode);
            self.shapes.push(shape);
        }
        
        self.current_shape_idx = end;
        self.current_shape_idx >= self.max_shapes
    }
    
    pub fn get_json(&self) -> Result<String, JsValue> {
        let width = self.img_width as f64;
        let height = self.img_height as f64;
        let aspect = width / height;
        let ymin = -10.0;
        let ymax = 10.0;
        let xmax = 10.0 * aspect;
        let xmin = -xmax;
        
        use rand::Rng;
        let seed: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let mut final_expressions = Vec::new();
        
        final_expressions.push(desmos::Expression::Text(desmos::TextData {
            id: AUTHOR_ID.to_string(),
            text: "Made by Bennett Lang (Bennebotix)".to_string(),
        }));
        
        final_expressions.push(desmos::Expression::Text(desmos::TextData {
            id: INSTRUCTIONS_ID.to_string(),
            text: "Unhide the folder to see the image (may be laggy)".to_string(),
        }));
        
        final_expressions.push(desmos::Expression::Text(desmos::TextData {
            id: GITHUB_ID.to_string(),
            text: "This was made using EIDOS, a simple webapp using Rust in WebAssembly.\n\nYou can check it out here:\nhttps://github.com/Bennebotix/EIDOS".to_string(),
        }));
        
        final_expressions.push(desmos::Expression::Folder(desmos::FolderData {
            id: FOLDER_ID.to_string(),
            title: FOLDER_TITLE.to_string(),
            hidden: true,
            collapsed: true,
        }));
        
        for (i, shape) in self.shapes.iter().enumerate() {
            let cx = (shape.x / width) * (20.0 * aspect) - (10.0 * aspect);
            let cy = -((shape.y / height) * 20.0 - 10.0);
            
            let scale_factor = 20.0 / height;
            let rx = shape.rx * scale_factor;
            let ry = shape.ry * scale_factor;
            let rot = -shape.angle;
            
            let cos = rot.cos();
            let sin = rot.sin();
            
            let (r, g, b, a) = shape.color;
            let color_hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
            let opacity = format!("{:.3}", a as f64 / 255.0);
            
            let latex = format!(
                r#"\frac{{\left(\left(x-{cx:.3}\right)\cdot{c:.3}+\left(y-{cy:.3}\right)\cdot{s:.3}\right)^{{2}}}}{{{rx:.3}^{{2}}}}+\frac{{\left(\left(x-{cx:.3}\right)\cdot{s:.3}-\left(y-{cy:.3}\right)\cdot{c:.3}\right)^{{2}}}}{{{ry:.3}^{{2}}}}\le1"#,
                cx=cx, cy=cy, c=cos, s=sin, rx=rx, ry=ry
            );
            
            final_expressions.push(desmos::Expression::Expression(desmos::ExpressionData {
                id: format!("{}", i + MIN_SHAPE_ID),
                folder_id: Some(FOLDER_ID.to_string()),
                color: color_hex,
                latex,
                fill: Some(true),
                lines: Some(false),
                fill_opacity: Some(opacity),
                line_width: Some("0".to_string()),
                domain: None,
                parametric_domain: None,
            }));
        }

        let state = DesmosState {
            version: 11,
            random_seed: seed,
            graph: GraphSettings {
                viewport: Viewport { xmin, ymin, xmax, ymax },
            },
            expressions: ExpressionList { list: final_expressions }, 
            include_function_parameters_in_random_seed: true,
            do_not_migrate_movable_point_style: true,
        };
        
        serde_json::to_string(&state).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
