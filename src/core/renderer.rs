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
    // Get values for each equation at each x pixel.
    let mut values = vec![];
    for pixel_x in 0..data.resolution.x {
        values.push(vec![]);
        let x1 = data.position.x as f32 + (data.size.x as f32 * (pixel_x as f32 / data.resolution.x as f32));
        let x2 = data.position.x as f32 + (data.size.x as f32 * ((pixel_x as f32 + 1.0) / data.resolution.x as f32));
        for index in 0..data.equations.len() {
            let equation = &data.equations[index];
            let r1       = equation.evaluate(x1);
            let r2       = equation.evaluate(x2);
            if ! r1.success {
                return RendererResult {
                    success         : false,
                    export_filename : "".to_string(),
                    exception       : r1.exception
                };
            } else if ! r2.success {
                return RendererResult {
                    success         : false,
                    export_filename : "".to_string(),
                    exception       : r2.exception
                };
            }
            values[pixel_x as usize].push(data::MinMax {
                min : r1.value,
                max : r2.value
            });
        }
    }
    // Draw equation values to image.
    for (pixel_x, pixel_y_reversed, pixel) in buffer.enumerate_pixels_mut() {
        let pixel_y = data.resolution.y - (pixel_y_reversed as i32);
        let y1 = data.position.y as f32 + (data.size.y as f32 * (pixel_y as f32 / data.resolution.y as f32));
        let y2 = data.position.y as f32 + (data.size.y as f32 * ((pixel_y as f32 + 1.0) / data.resolution.y as f32));

        let mut colour   = data::Colour {
            r : 1.0,
            g : 1.0,
            b : 1.0,
            a : 1.0
        };
        let mut done = false;
        for index in 0..data.equations.len() {
            let r1       = values[pixel_x as usize][index].min.clone();
            let r2       = values[pixel_x as usize][index].max.clone();
            for r1i in 0..r1.values.len() {
                let r1v = r1.values[r1i];
                for r2i in 0..r2.values.len() {
                    let r2v = r2.values[r2i];
                    if (r1v > y1 && r2v < y2) || (r2v > y1 && r1v < y2) {
                        colour.r = 0.0;
                        colour.g = 0.0;
                        colour.b = 0.0;
                        done = true;
                    }
                    if done {
                        break;
                    }
                }
                if done {
                    break;
                }
            }
            if done {
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
