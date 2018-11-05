extern crate image;

use std::fs::File;
use std::path::Path;

fn main() -> std::io::Result<()>
{
    let image_path = Path::new("out/image.png");
    let file = File::create(&image_path)?;
    let png_encoder = image::png::PNGEncoder::new(file);

	const NX : usize = 200;
	const NY : usize = 100;
    let mut pixels : Vec<u8> = vec![0; NX * NY * 3];

	for i in 0..NX {
		for j in 0..NY {
			let r  = i as f32 / NX as f32;
            let g  = j as f32 / NY as f32;
            let b  = 0.2;
            let ir = (255.99*r) as u8;
            let ig = (255.99*g) as u8;
            let ib = (255.99*b) as u8;
            pixels[i*3 + j*NX*3] = ir;
            pixels[1 + i*3 + j*NX*3] = ig;
            pixels[2 + i*3 + j*NX*3] = ib;
		}
	}

    png_encoder.encode(&pixels, NX as u32, NY as u32, image::ColorType::RGB(8))?;

    Ok(())
}