extern crate image;

use std::fs::File;
use std::path::Path;

struct Vec3<T> {
    a: T,
    b: T,
    c: T
}

impl<T> Vec3<T> {
    fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { a: x, b: y, c: z}
    }

    fn x(&self) -> &T {
        &self.a
    }

    fn y(&self) -> &T {
        &self.b
    }

    fn z(&self) -> &T {
        &self.c
    }

    fn r(&self) -> &T {
        &self.a
    }

    fn g(&self) -> &T {
        &self.b
    }

    fn b(&self) -> &T {
        &self.c
    }
}

fn main() -> std::io::Result<()>
{
    let v = Vec3 {a: 1, b: 2, c: 3};
	const NX : usize = 200;
	const NY : usize = 100;
    let mut pixels : Vec<u8> = vec![0; NX * NY * 3];
	for i in 0..NX {
		for j in 0..NY {
            let col = Vec3::new(i as f32 / NX as f32, j as f32 / NY as f32, 0.2);
            let ir = (255.99*col.r()) as u8;
            let ig = (255.99*col.g()) as u8;
            let ib = (255.99*col.b()) as u8;
            pixels[i*3 + j*NX*3] = ir;
            pixels[1 + i*3 + j*NX*3] = ig;
            pixels[2 + i*3 + j*NX*3] = ib;
		}
	}

    write_image("out/archivo.png", &pixels, NX, NY)?;;

    Ok(())
}

fn write_image(filepath: &str, pixels: &[u8], x_size: usize, y_size: usize) -> std::io::Result<()>
{
    let image_path = Path::new(filepath);
    let file = File::create(&image_path)?;
    let png_encoder = image::png::PNGEncoder::new(file);
    png_encoder.encode(&pixels, x_size as u32, y_size as u32, image::ColorType::RGB(8))?;
    Ok(())
}