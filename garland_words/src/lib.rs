extern crate unicode_segmentation as us;
use us::UnicodeSegmentation;

pub trait Garland {
    /// Gives the "garland" degree.
    fn garland(&self) -> usize;
}

impl<'a> Garland for &'a str {
    fn garland(&self) -> usize {
        let mut degree = 0;
        let chars: Vec<_> = self.graphemes(true).collect();
        let len = chars.len();
        for i in 0..len {
            if chars[0..i] == chars[len-i..len] {
                degree = i;
            }
        }
        degree
    }
}

#[cfg(test)]
mod tests {
    use Garland;
    #[test]
    fn onion() {
        assert_eq!("onion".garland(), 2);
    }
    #[test]
    fn ceramic() {
        assert_eq!("ceramic".garland(), 1);
    }
    #[test]
    fn programmer() {
        assert_eq!("programmer".garland(), 0);
    }
    #[test]
    fn alfalfa() {
        assert_eq!("alfalfa".garland(), 4);
    }
}
