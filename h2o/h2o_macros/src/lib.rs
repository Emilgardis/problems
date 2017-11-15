#[macro_export]
macro_rules! {
    ($($elem:tt)*) => {
        as_item!(
        pub enum Element {
            $($elem)*
        }
        )
    }
}

macro_rules! as_item { ($i:item) => {$i} }
