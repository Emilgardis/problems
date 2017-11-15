error_chain!{
    errors {
        NoZeroNumeral {
            description("There is no zero roman numeral.")
        }
        InvalidSequence(seq: &'static str) {
            description("Invalid sequence")
            display("Invalid sequence: {}", seq)
        }
    }
    foreign_links {
       FmtErr(::std::fmt::Error); 
    } 
}
