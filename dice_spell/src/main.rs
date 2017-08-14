//! Daily programmer challenge #326
//! Multifaceted alphabet blocks
//! 
//! https://redd.it/6t0zua

//! Histogram


extern crate clap;
#[macro_use]
extern crate error_chain;

mod errors;

use errors::*;

use std::fmt;

use std::collections::HashMap;

quick_main!(run);

#[derive(PartialEq, Eq, Hash)]
pub struct Letter<> {
    pub base: char,
    pub repeat: usize,
}

impl fmt::Debug for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'")?;
        for _ in  0..self.repeat {
            write!(f, "{}", self.base)?;
        }
        write!(f, "'")
    }
}

impl Letter {
    /// Asserts if letter is of this base.
    /// 
    /// Make case agnostic
    pub fn is_base(&self, ch: char) -> bool {
        self.base == ch
    }

    pub fn new(base: char, repeat: usize) -> Letter {
        Letter {
            base,
            repeat,
        }
    }
}
#[derive(Debug)]
pub struct Histogram<T>
    where T: std::cmp::Eq +
            std::hash::Hash + 
            std::fmt::Debug
{
    pub map: HashMap<T, u32>,
}

impl<T> Histogram<T> 
    where T: std::cmp::Eq +
            std::hash::Hash +
            std::fmt::Debug
{
    pub fn new() -> Histogram<T> {
        Histogram {
            map: HashMap::new()
        }
    }
    /// Returns the most occurring character(s).
    pub fn get_maximum(&self) -> Option<(Vec<&T>, u32)> {
        if self.map.is_empty() {
            return None
        }
        let mut max_vec: Vec<&T> = vec![];
        let mut max = 0;
        for (key, val) in self.map.iter() {
            if val == &max {
                max_vec.push(key);
            } else if val > &max {
                max_vec.clear();
                max_vec.push(key);
                max = *val;
            }
        }
        Some((max_vec, max))
    }
}

impl Histogram<char> {
    pub fn include_word(&mut self, word: &str) -> Result<()>{
        // Get occurancess of each letter in the alphabet-.

        for ch in word.chars() {
            if !ch.is_lowercase() {
                bail!("word needs to be lowercase, got uppercase letter {:?}", ch);
            }
            *self.map.entry(ch).or_insert(0) += 1;
        }
        Ok(())
    }
}

/// Not really a histogram.
// Dunno if this is really helpful.
impl Histogram<Letter> {
    // Much alike Histogram<char>, but now counts repeats.
    pub fn include_repeats(&mut self, word: &str) -> Result<()>{
        // FIXME: We are allocating a new BTreeMap/histogram for each word, is this needed?
        let mut hist: Histogram<char> = Histogram::new();
        hist.include_word(word)?;
        for (key, value) in hist.map.iter() {
            *self.map.entry(Letter::new(*key, *value as usize)).or_insert(0) += 1;
        }
        Ok(())
    }

    pub fn get_maximum_of_grade(&self, n: usize) -> Option<(Vec<&Letter>, u32)> {
        if self.map.is_empty() || !self.map.keys().any(|f| f.repeat == n){
            return None
        }
        let mut max_vec: Vec<_> = vec![];
        let mut max = 0;
        for (key, val) in self.map.iter() {
            if key.repeat != n {
                continue;
            }
            if val == &max {
                max_vec.push(key);
            } else if val > &max {
                max_vec.clear();
                max_vec.push(key);
                max = *val;
            }
        }
        Some((max_vec, max))
    }

    pub fn get_grade(&self, n: usize) -> Option<Vec<(&Letter, u32)>> {
        if self.map.is_empty() || !self.map.keys().any(|f| f.repeat == n) {
            return None
        }

        let mut vec = vec![];

        for (key, val) in self.map.iter() {
            if key.repeat != n {
                continue;
            }
            vec.push((key, *val))
        }
        Some(vec)
    }
}

#[derive(Debug)]
pub struct Block {
    faces: Vec<char>,
    n: usize,
}

impl Block {
    pub fn new(n: usize) -> Block {
        Block {
            faces: Vec::with_capacity(n),
            n,
        }
    }

    pub fn single(ch: char) -> Block {
        let mut block = Block::new(1);
        block.push(ch).expect("Cannot happen");
        block
    }
    
    pub fn push(&mut self, ch: char) -> Result<()> {
        if self.faces.len() < self.n {
            self.faces.push(ch);
            Ok(())
        } else {
            bail!("Can't add more faces. len: {:?}, n: {:?}", self.faces.len(), self.n)
        }
    }

    pub fn faces(&self) -> &[char] {
        self.faces.as_slice()
    }

    pub fn faces_mut(&mut self) -> &mut[char] {
        self.faces.as_mut_slice()
    }
}


pub struct BlockBuilder {
    hist: Histogram<char>,
    repeats: Histogram<Letter>,
}

impl BlockBuilder {
    pub fn new(hist: Histogram<char>, repeats: Histogram<Letter>) -> BlockBuilder {
        BlockBuilder {
            hist,
            repeats,
        }
    }

    pub fn generate(self) -> Result<Vec<Block>> {
        let mut vec = vec![];
        let mut max_vec = self.hist.get_maximum().ok_or("Histogram not filled")?.0;
        max_vec.as_mut_slice().sort_unstable();
        vec.push(Block::single(**max_vec.get(0).unwrap()));



        Ok(vec)
    }
}


fn run() -> Result<()> {
    use std::fs::File;
    use std::io::Read;

    let words = vec!["one", "two", "three", "four", "five", "six", "seven"];
    //let mut buf = String::new();
    //File::open("words.txt")?.read_to_string(&mut buf)?;
    //let words = buf.split_whitespace().collect::<Vec<_>>();
    let mut hist: Histogram<char> = Histogram::new();
    let mut repeats: Histogram<Letter> = Histogram::new();

    for word in words {
        hist.include_word(word).chain_err(|| format!("While proccessing \"{}\"", word))?;
        repeats.include_repeats(word).unwrap();
    }
    // Make dies/blocks.
    println!("histogram: {:?}\nrepeats: {:?}", hist, repeats);
    println!("histogram max: {:?}", hist.get_maximum());
    println!("repeats:\n 1 {:?}\n 2 {:?}\n 3 {:?}\n 4 {:?}", repeats.get_maximum_of_grade(1), repeats.get_maximum_of_grade(2), repeats.get_maximum_of_grade(3), repeats.get_maximum_of_grade(4));
    println!("order: \n 1 {:?}\n 2 {:?}\n 3 {:?}\n 4 {:?}", repeats.get_grade(1), repeats.get_grade(2),repeats.get_grade(3),repeats.get_grade(4));


   println!("result: {:?}", BlockBuilder::new(hist, repeats).generate()); 
   Ok(())
}
