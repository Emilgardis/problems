extern crate num;
extern crate image;

mod mandelbrot_gen;

pub use mandelbrot_gen::{ComplexPlace, MandelbrotGenerator};

pub mod image_utils {
    use image;
    use num::complex::Complex64;
    use super::mandelbrot_gen::{MandelbrotGenerator, ComplexPlace};
    use image::ImageBuffer;
    pub fn generate_rgb(gen: &MandelbrotGenerator, x: u32,y: u32) -> image::Rgb<u8> {
        let c_re = gen.x_to_re(x as usize);
        let c_im = gen.y_to_im(y as usize);
        let c = Complex64::new(c_re, c_im);
        use self::ComplexPlace::*;
        match gen.compute_iterations(c) {
            Escapes(z,i) => {
                image::Rgb( match i % 16 {
                     0 => [66, 30, 15], 
                     1 => [25, 7, 26],
                     2 => [9,1, 47],
                     3 => [4,4,73],
                     4 => [0, 7,100],
                     5 => [12, 44, 138],
                     6 => [24, 82, 177],
                     7 => [57, 125, 209],
                     8 => [134, 181, 229],
                     9 => [211, 236, 248],
                    10 => [241, 233, 191],
                    11 => [248, 201, 95],
                    12 => [255, 170, 0],
                    13 => [204, 128, 0],
                    14 => [153,  87, 0],
                    15 => [106, 52, 3],
                    _ => unreachable!()
                })
            }
            _ => image::Rgb([0,0,0]),
        }
    }
}
