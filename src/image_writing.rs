use glam::Vec3;

use std::io::{self};

pub fn write_as_png(
    file_name: &str,
    pixels: &[Vec<Vec3>],
    width: u32,
    height: u32,
) -> Result<(), io::Error> {
    let mut img = image::ImageBuffer::new(width, height);

    for (y, row) in pixels.iter().enumerate() {
        for (x, &fc) in row.iter().enumerate() {
            let c = fc.as_ivec3();
            let r = c.x.clamp(0, 255) as u8;
            let g = c.y.clamp(0, 255) as u8;
            let b = c.z.clamp(0, 255) as u8;
            img.put_pixel(x as u32, y as u32, image::Rgb([r, g, b]));
        }
    }

    img.save(format!("{}.png", file_name))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
