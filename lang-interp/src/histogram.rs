use std::collections::BTreeMap;
use std::string::String;
use std::fmt;


#[derive(Serialize, Deserialize)]
pub struct Histogram<Data> where Data: Ord {
    source: BTreeMap<Data, u64>,
    total_entries: u64,
}

impl<Data> Histogram<Data> where Data: Ord {
    pub fn new(source: BTreeMap<Data, u64>, total_entries: u64) -> Histogram<Data> {
        Histogram {
            source: source,
            total_entries: total_entries,
        }
    }

    pub fn to_ranking(self) -> Ranking<Data> {
        let mut stack = BTreeMap::new();

        for (ch, count) in self.source.into_iter() {
            stack.insert(ch, (count as f64 /self.total_entries as f64) * 100.0);
        }
        Ranking(stack)
    }
}

impl<Data> fmt::Debug for Histogram<Data> where Data: Ord + fmt::Debug {
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

pub trait ToHistogram<Data> where Data: Ord {
    fn to_histogram(self) -> Histogram<Data>;
}

impl ToHistogram<char> for String {
    fn to_histogram(self) -> Histogram<char> {
        let mut count = BTreeMap::new();
        let mut entries = 0;
        for c in self.chars() {
            entries += 1;
            *count.entry(c).or_insert(0) += 1;
        }
        Histogram::new(count, entries)
    }
}

// Not used currently
impl<T> ToHistogram<T> for Vec<T> where T: Ord {
    fn to_histogram(self) -> Histogram<T> {
        let mut count = BTreeMap::new();
        let mut entries = 0;
        for obj in self {
            entries += 1;
            *count.entry(obj).or_insert(0) += 1;
        }
        Histogram::new(count, entries)
    }
}
