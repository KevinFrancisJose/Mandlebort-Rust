extern crate image;
extern crate num_complex;

use std::env;

const WIDTH : u32 = 800;
const HEIGHT : u32  = 800;

const SCALEX : f32= 3.0 / WIDTH as f32;
const SCALEY : f32= 3.0 / HEIGHT as f32;

fn value(x : u32, y : u32, iter_num : u32) -> image::Rgb<u8> {

        let cx = y as f32 * SCALEX - 1.5;
        let cy = x as f32 * SCALEY - 1.5;
        
        let c = num_complex::Complex::new(cx, cy);
        let mut z = num_complex::Complex::new(0.0, 0.0);

        let mut i = 0;
            
        while i <= iter_num && z.norm() <= 2.0 {
            z = z * z + c;
            i += 1;            
        }

        if i < iter_num {
            image::Rgb([255, 0, 0])
        }
        else {
            image::Rgb([0, 0, 0])
        }

}

fn main() {

    let args: Vec<String> = env::args().collect();

    let iter_num : u32 = args[1].parse().unwrap();


    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);


    for x in 0..WIDTH {
        for y in 0..HEIGHT {

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = value(x, y, iter_num);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}