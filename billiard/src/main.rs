//! In a billiard table with horizontal side a inches and vertical side b inches, a ball is
//! launched from the middle of the table. After s>0 seconds the ball returns to the point from
//! which it was launched, after having made m bounces off the vertical sides and n bounces off the
//! horizontal sides of the table. Find the launching angle A (measured from the horizontal), which
//! will be between 0 and 90 degrees inclusive, and the initial velocity of the ball.
//!
//! Assume that the collisions with a side are elastic (no energy loss), and thus the velocity
//! component of the ball parallel to each side remains unchanged. Also, assume the ball has a
//! radius of zero. Remember that, unlike pool tables, billiard tables have no pockets.
//! # Input
//!
//! Input consists of a sequence of lines, each containing five nonnegative integers separated by
//! whitespace. The five numbers are: a
//! , b, s, m, and n, respectively. All numbers are positive integers not greater than 10000.
//!
//! Input is terminated by a line containing five zeroes.
//! # Output
//!
//! For each input line except the last, output a line containing two real numbers (rounded to
//! exactly two decimal places) separated by a single space. The first number is the measure of the
//! angle A
//! in degrees and the second is the velocity of the ball measured in inches per second, according
//! to the description above.
//!
//! # Sample Input
//! ```ignore text
//! 100 100 1 1 1
//! 200 100 5 3 4
//! 201 132 48 1900 156
//! 0 0 0 0 0
//! ```
//!
//! # Sample Output
//! ```ignore text
//! 45.00 141.42
//! 33.69 144.22
//! 3.09 7967.81
//! ```
use std::io;

fn read_numbers() -> Option<Environment> {
    let mut line = String::new();
    if let Ok(_) = io::stdin().read_line(&mut line) {
        let v: Vec<usize> = line.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        if v.iter().all(|v| *v==0) {
            return None;
        }
        return Some(Environment::new(v));
    }
    None
}

struct Environment{
    a: usize,
    b: usize,
    s: usize,
    m: usize,
    n: usize,
}

const PI: f32 = std::f32::consts::PI;
impl Environment {
    fn new(v: Vec<usize>) -> Environment {
        Environment {
            a: v[0],
            b: v[1],
            s: v[2],
            m: v[3],
            n: v[4],
        }
    }
    fn calculate(self) -> String {

        let width = self.a * self.m;
        let height = self.b * self.n;
        let dist = (width.pow(2) as f32 + height.pow(2) as f32).sqrt();
        let angle = 180.0/PI * (height as f32).atan2(width as f32);
        let vel = dist as f32 / self.s as f32;
        format!("{:.2} {:.2}", angle, vel)
    }
}

fn main() {
    while let Some(env) = read_numbers() {
        println!("{}", env.calculate())
    }
}
