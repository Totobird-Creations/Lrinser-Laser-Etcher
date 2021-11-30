use png;
use super::exceptions;
use super::interpreter;
use super::data;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
//use math::round::floor;



#[derive(Clone, Debug)]
pub struct RendererResult {
    pub success   : bool,
    pub exception : exceptions::RendererException
}


pub fn render(mut data : interpreter::InterpreterData) -> RendererResult {
    if data.resolution.x == 0 {
        data.resolution.x = data.size.x
    }
    if data.resolution.y == 0 {
        data.resolution.y = data.size.y
    }

    let         path                             = Path::new(&data.export);
    let         file                             = File::create(path).unwrap();
    let ref mut buffer                           = BufWriter::new(file);

    let mut     encoder                          = png::Encoder::new(buffer, data.resolution.x as u32, data.resolution.y as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let         src_chr                          = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(src_chr);
    let mut     writer                           = encoder.write_header().unwrap();
    let mut     image   : Vec<Vec<data::Colour>> = vec![vec![]];
    for pixel_y in 0..data.resolution.y {
        image.push(vec![]);
        for pixel_x in 0..data.resolution.x {
            let x1 = data.position.x as f32 + (data.size.x as f32 * (pixel_x as f32 / data.resolution.x as f32));
            let x2 = data.position.x as f32 + (data.size.x as f32 * ((pixel_x as f32 + 1.0) / data.resolution.x as f32));
            let y1 = data.position.y as f32 + (data.size.y as f32 * (pixel_y as f32 / data.resolution.y as f32));
            let y2 = data.position.y as f32 + (data.size.y as f32 * ((pixel_y as f32 + 1.0) / data.resolution.y as f32));

            //if floor(x as f64, 0) == floor(y as f64, 0) {
            if (x1 > y1 && x2 < y2) || (x2 > y1 && x1 < y2) {
                image[pixel_y as usize].push(data::colour(1.0, 0.0, 0.0, 1.0));
            } else {
                image[pixel_y as usize].push(data::colour(1.0, 1.0, 1.0, 1.0));
            }

        }
    }

    let mut     fin_img                          = vec![];
    for row in image {
        for pixel in row {
            fin_img.push((pixel.r * 255.0) as u8);
            fin_img.push((pixel.g * 255.0) as u8);
            fin_img.push((pixel.b * 255.0) as u8);
            fin_img.push((pixel.a * 255.0) as u8);
        }
    }
    let         fin_obj                          = &fin_img.as_slice();
    writer.write_image_data(fin_obj).unwrap();

    return RendererResult {
        success   : true,
        exception : exceptions::RendererException {
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