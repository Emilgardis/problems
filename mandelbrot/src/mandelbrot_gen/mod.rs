
use num::complex::Complex64;

use std::fmt;

/// Where is the complex number.
pub enum ComplexPlace {
    /// Max iterations reached, may not diverge.
    Max,
    // Period,
    /// Inside Cardioid, does not diverge.
    Cardioid,
    /// Inside Bulb, does not diverge.
    Bulb,
    /// Escaped after n iteration, diverges.
    Escapes(Complex64, usize),
}

impl fmt::Debug for ComplexPlace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ComplexPlace::*;
        let val = match *self {
                Max => "M".into(),
                Cardioid => "C".into(),
                Bulb => "B".into(),
                Escapes(_, iter) => {
                    if let Some(ref prec) = f.precision() {
                        format!("{}", 
                                (iter - 0) * (9 - 0)/ (prec - 0 ) + 0
                                )
                    } else {
                        panic!("No prec specified")
                    }
                },
        };
        write!(f, "{}", val)
    }
}

// complex/image space = [-2,1]
/// Mandelbrot
/// 
/// Formula
/// 
/// x_(k+1) = x^(2)_k - y^(2)_k + Re(c)
/// 
/// y_(k+1) = 2 x_k * y_k + Im(c)
/// 
/// Iterate until x*x + y*y > 2 or max iter has been reached
/// 
/// Note that mandelbrot is symmetric on x plane, i.e height/2 is only needed.
/// This means that 
/// ```math
/// color(x,y) = color(x,y)
/// ```
///  
/// # TODO:
/// * impl perturbation (see [2]¤Perturbation_Theory)
/// * use num::rational
///  
/// ### Sources:
/// * [1]: http://stackoverflow.com/questions/8381675/how-to-perform-simple-zoom-into-mandelbrot-set
/// * [2]: https://en.wikibooks.org/wiki/Fractals/Iterations_in_the_complex_plane/Mandelbrot_set
/// 
pub struct MandelbrotGenerator {
    c_max: Complex64,
    c_min: Complex64,
    width: usize,
    height: usize,
    re_factor: f64,
    im_factor: f64,
    origin: Complex64,
    pub escape: f64,
    pub max_iterations: usize,
}

impl MandelbrotGenerator {
    /// returns width, height
    pub fn get_image_space(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    ///
    ///
    ///# TODO
    pub fn set_image_space(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.re_factor = (self.c_max.re- self.c_min.re) / (width-1) as f64;
        self.im_factor = (self.c_max.im- self.c_min.im) / (height-1) as f64;
        //self.im_factor = (self.c_max.im- self.c_min.im) / (y_space[1]- y_space[0]-1) as f64;
    }
    pub fn zoom(&mut self, zoom_factor: f64, origin: Complex64) {
        self.origin = origin;
        self.c_max = self.c_max.unscale(zoom_factor);
        self.c_min = self.c_min.unscale(zoom_factor);
        self.re_factor = (self.c_max.re- self.c_min.re) / (self.width-1) as f64;
        self.im_factor = (self.c_max.im- self.c_min.im) / (self.height-1) as f64;

    }
    pub fn y_to_im(&self, y: usize) -> f64 {
        self.c_min.im + y as f64 *self.im_factor + self.origin.im
    //    ((y-self.y_space[0])/(self.y_space[1] - self.y_space[0]) * self.c_max.im - self.c_min.im) + self.c_min.im
    }

    pub fn x_to_re(&self, x: usize) -> f64 {
        self.c_min.re + x as f64 *self.re_factor + self.origin.re
    //    ((x-self.x_space[0])/(self.x_space[1] - self.x_space[0]) * self.c_max.re - self.c_min.re) + self.c_min.re
    }
    
    pub fn is_in_p2_bulb(c: &Complex64) -> bool {
        (c.re + 1.0).powf(2.0) + c.im.powf(2.0) < 1.0/16.0 
    }

    pub fn is_in_cardioid(c: &Complex64) -> bool {
        let q = (c.re - 1.0/4.0).powf(2.0) + c.im.powf(2.0);
        q * (q + (c.re -1.0/4.0)) < 1.0/4.0 * c.im.powf(2.0) 
    }

    /// Returns iterations of complex number
    pub fn compute_iterations(&self, c: Complex64) -> ComplexPlace {
        let mut z = c.clone();
        let mut iter = 0;
        //let mut c = c.clone();
        if MandelbrotGenerator::is_in_p2_bulb(&c) {
            return ComplexPlace::Bulb;
        } else if MandelbrotGenerator::is_in_cardioid(&c) {
            return ComplexPlace::Cardioid;
        }
        while z.norm_sqr() <= self.escape {
            if iter > self.max_iterations {
                return ComplexPlace::Max;
            }
            z = z.powf(2.0) + c;
            iter += 1;
        }
        ComplexPlace::Escapes(z, iter)
    }
}

impl Default for MandelbrotGenerator {
    fn default() -> Self {
        // Works like this
        let c_max = Complex64::new(0.5, 1.25); //   (lrightx, ulefty )
        let c_min = Complex64::new(-2.0, -1.25); // (uleftx,  lrighty)
        let width = 800;
        let height = 800;
        let re_factor = (c_max.re- c_min.re) / (width-1) as f64;
        let im_factor = (c_max.im- c_min.im) / (height-1) as f64;
        MandelbrotGenerator {
            c_max: c_max,
            c_min: c_min,
            width: width,
            height: height,
            origin: Complex64::new(0.,0.),
            re_factor: re_factor,
            im_factor: im_factor,
            escape: 4.0,
            max_iterations: 100,
        }
    } 
}



#[cfg(test)]
mod tests {

    use super::*;
    use num::complex::Complex64;
    #[test]
    fn basic_test() {
        let mut gen = MandelbrotGenerator::default();
        gen.max_iterations = 21;
        println!();
        gen.set_image_space(200, 100);
        //gen.set_image_space([0, 200], [0,100]);
        let (width, height) = gen.get_image_space();
        //let (width, height) = (200,100);
        for y in 0..height {
            let c_im = gen.y_to_im(y);
            for x in 0..width{
                let c_re = gen.x_to_re(x);
                let c = Complex64::new(c_re, c_im);
                let iterations = gen.compute_iterations(c);
                print!("{:.100?}", iterations);
            }
            println!();
        }
    }
}
