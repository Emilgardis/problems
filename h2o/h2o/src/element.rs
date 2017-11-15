#[macro_export]
macro_rules! elements_decl {
    ($($elem:tt)*) => {
        as_item!{
            pub enum Symbol {
                $($elem)*
            }
        }
    };
}

macro_rules! as_item { ($i:item) => {$i} }

symbol_decl!(H,He,Li,Be,B§);

// db: http://physics.nist.gov/cgi-bin/Compositions/stand_alone.pl?ele=&all=all&ascii=ascii2&isotype=some
// https://www.nist.gov/pml/atomic-weights-and-isotopic-compositions-relative-atomic-masses
// pdf: https://www.nist.gov/sites/default/files/documents/2017/02/02/periodic-table-2017-crop.pdf
pub struct ElementInfo {
    pub number: &'static usize,
    pub symbol: &'static Symbol,
    pub mass_number: &'static usize,
    pub relative_amass: &'static (f32, usize),
    pub aweight: &'static (f32, usize),
    // More, density, electro negativity, etc etc.
}


