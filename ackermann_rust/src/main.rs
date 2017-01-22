use std::io::{self, Write};
use std::rc::Rc;
fn main() {
    let std = io::stdout();
    let handle = std.lock();
    let mut ack = AckermannGen::new(4,3, handle);
    let ack_res = ack.run();
    println!("(4,3): {:?}", ack_res);
    println!("(4,3): {:?}", ack.total);

}

struct AckermannGen<T: Write> {
    handle: T,
    pub total: Rc<u64>,
    args: (u64, u64),
}

impl<T: Write> AckermannGen<T> {
    fn new(m: u64, n: u64, write: T) -> AckermannGen<T> {
        AckermannGen {
                handle: write,
                total: 0.into(),
                args: (m, n),
            }
        }

    #[inline]
    fn _ackermann(&mut self, m: &u64, n: &u64) -> Result<u64, ()> {
        // Do stuff
        match (m, n) {
            (m, n) if m == &0 => Ok(*n+1),
            (m, n) if n == &0 && m > &0 => self.ackermann(*m-1,1),
            (m, n) => {
                let new_n = self.ackermann(*m, *n-1).expect("Fail");
                self.ackermann(*m-1,new_n)
            }
        }
    }

    #[inline]
    pub fn ackermann(&mut self, m: u64, n: u64) -> Result<u64, ()> {
        {
            //let mut total = Rc::get_mut(&mut self.total).unwrap();
            //*total += 1;
        }
        //if *self.total.as_ref() % 50_000 == 0 {
        //    write!(self.handle, "{}: {:?} {:?}\r", self.total, m, n);
        //}
        self._ackermann(&m, &n)
    }

    pub fn run(&mut self) -> Result<u64, ()> {
        let (m,n) = (self.args.0, self.args.1);
        self.ackermann(m, n)
    }
}
