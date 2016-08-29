use std::collections::BTreeMap;
use std::string::String;
use std::fmt;


#[derive(Serialize, Deserialize, Clone)]
pub struct NGram<Data> where Data: Ord + Clone {
    source: BTreeMap<Data, u64>,
    total_entries: u64,
    n: u8,
}

impl<Data> NGram<Data> where Data: Ord + Clone {
    pub fn new(source: BTreeMap<Data, u64>, total_entries: u64, n: u8) -> NGram<Data> {
        NGram {
            source: source,
            total_entries: total_entries,
            n: n,
        }
    }

    pub fn as_ranking(&self) -> Ranking<Data> {
        let mut stack = BTreeMap::new();

        for (ch, count) in self.source.iter() {
            stack.insert(ch.clone(), (*count as f64 /self.total_entries as f64) * 100.0);
        }
        Ranking(stack)
    }

    pub fn append(&mut self, other: NGram<Data>) {
        for (ch, count) in other.source.iter() {
            *self.source.entry(ch.clone()).or_insert(1) += count.clone();
        }
        self.total_entries += other.total_entries;
    }
}

impl<Data> fmt::Debug for NGram<Data> where Data: Ord + fmt::Debug + Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.source)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ranking<Data: Ord + Sized>(BTreeMap<Data, f64>);

impl<Data> Ranking<Data> where Data: Ord {
    pub fn similarity(&self, other: &Ranking<Data>) -> f64 {
        let mut fitness = 0.0;
        for (data, p1) in &self.0 {
            if let Some(p2) = other.0.get(&data) {
                fitness += (p1 - p2).abs();
            } else {
                fitness += *p1;
            }
        }
        fitness
    }
}

pub trait ToNGram<Data> where Data: Ord + Clone {
    fn to_ngram(self, n: u8) -> NGram<Data>;
}

impl ToNGram<String> for String {
    fn to_ngram(self, n: u8) -> NGram<String> {
        let n = n.clone();
        if n == 1 {
            let mut count = BTreeMap::new();
            let mut entries = 0;
            for c in self.chars() {
                if c.is_alphabetic() {
                    let c_lc = c.to_lowercase().collect();
                    entries += 1;
                    *count.entry(c_lc).or_insert(0) += 1;
                }
            }
            NGram::new(count, entries, n)
        } else if n == 2 {
            let mut count = BTreeMap::new();
            let mut entries = 0;
            //let mut filtered: String = self.chars().filter(|a| a.is_alphabetic() || a.is_whitespace()).collect();
            let mut c_iter = self.chars().peekable();
            while let Some(c) = c_iter.next() {
                let other = match c_iter.peek() {
                    Some(c) => c,
                    None => continue,
                };
                if c.is_whitespace() || !c.is_alphabetic() {
                    continue;
                }
                if other.is_alphabetic() {
                    let cs_lc: String = format!("{}{}", c, other).to_lowercase().into();
                    entries += 1;
                    *count.entry(cs_lc).or_insert(0) += 1;
                } else {
                    continue;
                }
            }
            NGram::new(count, entries, n)
        } else {
            unimplemented!()
        }
    }
}

// Not used currently
impl<T> ToNGram<T> for Vec<T> where T: Ord + Clone {
    fn to_ngram(self, n: u8) -> NGram<T> {
        unimplemented!();
        let mut count = BTreeMap::new();
        let mut entries = 0;
        for obj in self {
            entries += 1;
            *count.entry(obj).or_insert(0) += 1;
        }
        NGram::new(count, entries, n)
    }
}
