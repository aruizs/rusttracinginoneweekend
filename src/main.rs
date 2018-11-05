use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()>
{
    let image_path = Path::new("out/image.ppm");
    let mut file = File::create(&image_path)?;

	const NX : u32 = 200;
	const NY : u32 = 100;
	write!(file, "P3\n{} {}\n255\n", NX, NY);
	for j in (0..NY).rev() {
		for i in 0..NX {
			let r  = i as f32 / NX as f32;
            let g  = j as f32 / NY as f32;
            let b  = 0.2;
            let ir = (255.99*r) as i32;
            let ig = (255.99*g) as i32;
            let ib = (255.99*b) as i32;
            write!(file, "{} {} {}\n", ir, ig, ib)?;
		}
		
	}

    file.sync_all()?;
    Ok(())
}