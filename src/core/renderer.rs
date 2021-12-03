use image;
use super::exceptions;
use super::interpreter;
use super::data;



// Success/Failure identification class.
#[derive(Clone, Debug)]
pub struct RendererResult {
    pub success         : bool,
    pub export_filename : String,
    pub exception       : exceptions::RendererException
}


// Start rendering.
pub fn render(mut data : interpreter::InterpreterData) -> RendererResult {
    // If resolution is not set, use frame size.
    if data.resolution.x == 0 {
        data.resolution.x = data.size.x
    }
    if data.resolution.y == 0 {
        data.resolution.y = data.size.y
    }

    // Create image
    let mut buffer = image::ImageBuffer::new(data.resolution.x as u32, data.resolution.y as u32);
    /*for index in 0..data.equations.len() {
        for pixel_x in 0..data.resolution.x {
            let x1 = data.position.x as f32 + (data.size.x as f32 * (pixel_x as f32 / data.resolution.x as f32));
            let x2 = data.position.x as f32 + (data.size.x as f32 * ((pixel_x as f32 + 1.0) / data.resolution.x as f32));
            let equation = &data.equations[index];
            let r1       = equation.evaluate(x1);
            let r2       = equation.evaluate(x2);
            if ! r1.success {
                panic!("failed");
            } else if ! r2.success {
                panic!("failed");
            }
        }
    }*/
    for (pixel_x, pixel_y, pixel) in buffer.enumerate_pixels_mut() {
        let x1 = data.position.x as f32 + (data.size.x as f32 * (pixel_x as f32 / data.resolution.x as f32));
        let x2 = data.position.x as f32 + (data.size.x as f32 * ((pixel_x as f32 + 1.0) / data.resolution.x as f32));
        let y1 = data.position.y as f32 + (data.size.y as f32 * (pixel_y as f32 / data.resolution.y as f32));
        let y2 = data.position.y as f32 + (data.size.y as f32 * ((pixel_y as f32 + 1.0) / data.resolution.y as f32));

        let mut colour   = data::Colour {
            r : 1.0,
            g : 1.0,
            b : 1.0,
            a : 1.0
        };
        for index in 0..data.equations.len() {
            let equation = &data.equations[index];
            let r1       = equation.evaluate(x1);
            let r2       = equation.evaluate(x2);
            if (r1.value > y1 && r2.value < y2) || (r2.value > y1 && r1.value < y2) {
                colour.g = 0.0;
                colour.b = 0.0;
                break;
            }
        }
        *pixel = image::Rgba([(colour.r * 255.0) as u8, (colour.g * 255.0) as u8, (colour.b * 255.0) as u8, (colour.a * 255.0) as u8]);
    }

    // Write image.
    buffer.save(data.export.clone());

    // Return success.
    return RendererResult {
        success         : true,
        export_filename : data.export,
        exception       : exceptions::RendererException {
            base    : exceptions::RendererExceptionBase::NoException,
            message : "".to_string(),
            range   : data::Range {
                filename : "".to_string(),
                start    : 0,
                end      : 0
            }
        }
    };
}
