use image;
use super::logger;
use super::exceptions;
use super::interpreter;
use super::data;
use super::nodes;



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

    // Create image buffer.
    let mut buffer = image::ImageBuffer::new(data.resolution.x as u32, data.resolution.y as u32);

    // Get values for each equation at each x pixel.
    // This is a Vec (each column) containing a Vec (each valid equation set) containing a LeftRight (left and right side values of the column)
    let mut values : Vec<Vec<data::LeftRight>> = vec![]; 
    for pixel_x in 0..data.resolution.x {
        values.push(vec![]);
        let x1 = data.position.x as f32 + (data.size.x as f32 * (pixel_x as f32 / data.resolution.x as f32));
        let x2 = data.position.x as f32 + (data.size.x as f32 * ((pixel_x as f32 + 1.0) / data.resolution.x as f32));
        for index in 0..data.equations.len() {
            let equation = &data.equations[index];
            let eq1       = equation.simplify(x1);
            let eq2       = equation.simplify(x2);
            if ! eq1.success {
                return RendererResult {
                    success         : false,
                    export_filename : "".to_string(),
                    exception       : eq1.exception
                };
            } else if ! eq2.success {
                return RendererResult {
                    success         : false,
                    export_filename : "".to_string(),
                    exception       : eq2.exception
                };
            }
            match eq1.value.base.clone() {
                nodes::NodeBase::MultipleNumber {value : value1} => {
                    match eq2.value.base.clone() {
                        nodes::NodeBase::MultipleNumber {value : value2} => {
                            let values_index = values.len() - 1;
                            values[values_index].push(data::LeftRight {
                                left  : value1,
                                right : value2
                            });
                            continue;
                        },
                        _                                                 => {
                            logger::error(format!("Failed to simplify: {}", eq2.value));
                        }
                    }
                }
                _                                        => {
                    logger::error(format!("Failed to simplify: {}", eq1.value));
                }
            }
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
        for leftright in values[pixel_x as usize].clone() {
            for left in leftright.left.values.clone() {
                for right in leftright.right.values.clone() {
                    if (y1 >= left && y2 <= right) || (y1 <= left && y2 >= right) {
                        colour.r = 0.0;
                        colour.g = 0.0;
                        colour.b = 0.0;
                        done = true;
                        break;
                    }
                }
                if done {break}
            }
            if done {break}
        }

        *pixel = image::Rgba([(colour.r * 255.0) as u8, (colour.g * 255.0) as u8, (colour.b * 255.0) as u8, (colour.a * 255.0) as u8]);
    }

    // Write image.
    match buffer.save(data.export.clone()) {
        Ok(_v)  => (),
        Err(_e) => {
            logger::critical("Image write failed.");
            panic!();
        }
    };

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
