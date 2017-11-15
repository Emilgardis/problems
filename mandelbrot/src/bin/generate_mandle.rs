extern crate rayon;
extern crate num;
extern crate mandelbrot;
extern crate image;
extern crate clap;

use clap::{App};

use mandelbrot::image_utils::generate_rgb;
use num::complex::Complex64;
use mandelbrot::{MandelbrotGenerator, ComplexPlace};
use image::ImageBuffer;
use rayon::prelude::*;
fn xy_from_index(index: usize, width: usize) -> (u32, u32) {
    let x = index % width;
    let y = (index -x) / width;
    (x as u32,y as u32)
}
fn rgb_from_index(gen: &MandelbrotGenerator, index: usize, width: usize) -> image::Rgb<u8> {
    let (x,y) = xy_from_index(index, width);
    generate_rgb(gen, x, y)
}

fn make_image<F>(width: usize, height: usize, pre: F) where F: Fn(&mut MandelbrotGenerator) {
    let mut gen = MandelbrotGenerator::default();
    gen.set_image_space(width, height);
    gen.max_iterations = 4096;
    //let origin_re = -1.985_540_371_654_130_485_531_439_267_191_269_851_811_165_434_636_382_820_704_394_766_801_377;
    //let origin_im =  0.000_000_000_000_000_000_000_000_000_001_565_120_217_211_466_101_983_496_092_509_512_479_178;
    //let origin_re = -1.985;
    //let origin_im =  0.000_000_000_000_000_000_000_000_000_001_565_120;
    //let origin = Complex64::new(origin_re, origin_im);
    //let origin = Complex64::new(0.,0.);
    //gen.zoom(1.5, origin);
    let mut img: image::RgbImage = ImageBuffer::new(width as u32, height as u32);
    let mut ve = Vec::new();
    (0..(width*height)).into_par_iter().map(|ind| rgb_from_index(&gen, ind, width)).collect_into(&mut ve);
    println!("Generated image");
    for i in 0..(width*height) {
        let (x,y) = xy_from_index(i, width);
        img.put_pixel(x,y, ve[i]);
    }
    //println!("{:?}", pix_buf);
    img.save("basic.png").expect("Couldn't generate picture");
    println!("Saved image");
}

pub fn main() {
    make_image(1000, 1000, |g| g.zoom(1.5, Complex64::new(0.1, 0.0)));
}
