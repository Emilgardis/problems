#![cfg_attr(not(feature = "std"), no_std)]
#![feature(str_checked_slicing)]
#[macro_use]
extern crate serde_derive;
extern crate serde;

pub trait MorseDict {
    type Error;
    fn char_to_morse(&self, c: &char, buf: &mut [MorseCode]) -> Result<usize, Self::Error>;
    fn morse_to_char(&self, morse: &[MorseCode]) -> Result<char, Self::Error>;
}

static STANDARD_LOOKUP: &'static str = r##"ETIANMSURWDKGOHVFÜLÄPJBXCYZQÖĤ54Ŝ3É Ð2 È+ ÞÀĴ16=/ Ç Ĥ 7 ĜÑ8 90
          ?_    \"  .    @   '  -        ;! (     ,   :"##;
pub struct MorseStandard;

impl MorseDict for MorseStandard {
    type Error = ();
    fn char_to_morse(&self, c: &char, buf: &mut [MorseCode]) -> Result<usize, Self::Error> {
        use self::MorseCode::*;
        use self::MorseCode::IntraGap as G;
        use self::MorseCode::LetterGap as L;
        if c == &' ' {
            return Err(());
        }
        debug_assert!(buf.len() >= 6 * 2); // The depth of the lookup table is 6. times 2 since we include the gaps.
        let mut buf_ind = 0;
        let mut ind: usize = STANDARD_LOOKUP.chars().position(|s| s == *c).ok_or(())? + 2;
        let mut was_dash = true;
        loop {
            if ind % 2 == 0 {
                buf[buf_ind] = MorseCode::Dot;
            } else {
                buf[buf_ind] = MorseCode::Dash;
            }
            ind = ind / 2;
            buf_ind += 1;
            if ind != 1 {
                buf[buf_ind] = MorseCode::IntraGap;
            } else {
                buf[buf_ind] = MorseCode::LetterGap;
                break;
            }
            buf_ind += 1;
        }
        buf[0..buf_ind].reverse();
        Ok(buf_ind + 1)

    }

    fn morse_to_char(&self, morse: &[MorseCode]) -> Result<char, Self::Error> {
        let mut ind = 1usize;
        let mut iter = morse.iter();
        while let Some(code) = iter.next() {
            match *code {
                MorseCode::Dot => {
                    ind = 2 * ind;
                }
                MorseCode::Dash => {
                    ind = (ind * 2) + 1;
                }
                _ => return Err(()),
            }
            let next = iter.next();
            match *next.unwrap_or(&MorseCode::LetterGap) {
                MorseCode::Dot | MorseCode::Dash => return Err(()),
                MorseCode::IntraGap => {}
                MorseCode::LetterGap => {}
                MorseCode::WordGap => return Err(()),
            }
        }
        match STANDARD_LOOKUP.chars().skip(ind - 2).next().ok_or(())? as char {
            ' ' => Err(()),
            c => Ok(c),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MorseCode {
    Dot, // 1 = 1x
    Dash, // 111 = 3x
    IntraGap, // 0 = 1x
    LetterGap, // 000 = 3x
    WordGap, // 0000000 = 7x
}

impl MorseCode {
    pub fn as_character(&self) -> Option<char> {
        use self::MorseCode::*;
        match *self {
            Dot => Some('.'),
            Dash => Some('-'),
            IntraGap => None,
            LetterGap => Some(' '),
            WordGap => Some('/'),
        }
    }
}

pub trait Morsify {
    type Error;
    fn to_morse<M>(&self, dict: M, buf: &mut [MorseCode]) -> Result<usize, Self::Error>
    where
        M: MorseDict;
    fn to_morse_standard(&self, buf: &mut [MorseCode]) -> Result<usize, Self::Error> {
        Morsify::to_morse(self, MorseStandard, buf)
    }
    #[cfg(feature = "std")]
    fn to_morse_string<M>(&self, dict: M) -> Result<String, Self::Error>
    where
        M: MorseDict,
    {
        //TODO: Make this much better memory wise.
        let size = 100 * 10; // Generally we'd need 8 places for each char, but sometimes it's even more.
        let mut buf = vec![MorseCode::Dot; size];
        let mut char_buf = [0; 2];
        let mut string = String::new();
        let end = Morsify::to_morse(self, dict, &mut buf)?;

        for code in buf[0..end].iter() {
            if let Some(s) = code.as_character() {
                string = string + s.encode_utf8(&mut char_buf);
            }
        }
        Ok(string)
    }
    #[cfg(feature = "std")]
    fn to_morse_vec<M>(&self, dict: M) -> Result<Vec<MorseCode>, Self::Error>
    where
        M: MorseDict,
    {
        let size = 100 * 10; // Generally we'd need 6*2 places for each char, but sometimes it's even more.
        let mut buf = vec![MorseCode::Dot; size];
        let mut char_buf = [0; 2];
        let end = Morsify::to_morse(self, dict, &mut buf)?;
        buf.drain(end..);
        Ok(buf)
    }
}


impl Morsify for str {
    type Error = ();
    fn to_morse<M>(&self, dict: M, buf: &mut [MorseCode]) -> Result<usize, Self::Error>
    where
        M: MorseDict,
    {
        let mut ind = 0;
        let mut inner_buffer = [MorseCode::Dot; 20];
        for ch in self.chars() {
            if ch == ' ' {
                if let Some(mut prev) = buf.get_mut(ind - 1) {
                    if *prev == MorseCode::LetterGap {
                        *prev = MorseCode::WordGap;
                        continue;
                    }
                }
                buf[ind] = MorseCode::WordGap;
                ind += 1;
                continue;
            }
            let size = dict.char_to_morse(&ch, &mut inner_buffer).map_err(|e| ())?;
            buf[ind..ind + size].clone_from_slice(&inner_buffer[0..size]);
            ind += size;
        }
        Ok(ind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        use self::MorseCode::*;
        let example = "AAAAA";
        let mut buf = [MorseCode::Dot; 30];
        let end = example.to_morse_standard(&mut buf).unwrap();
        assert!(end == 20);
        assert_eq!(
            &buf[0..end],
            &[
                Dot,
                IntraGap,
                Dash,
                LetterGap,
                Dot,
                IntraGap,
                Dash,
                LetterGap,
                Dot,
                IntraGap,
                Dash,
                LetterGap,
                Dot,
                IntraGap,
                Dash,
                LetterGap,
                Dot,
                IntraGap,
                Dash,
                LetterGap,
            ]
        );
    }
    #[test]
    #[cfg(feature = "std")]
    fn strings() {
        assert_eq!(
            ".- -... -.-. -.. . ",
            "ABCDE".to_morse_string(MorseStandard).unwrap()
        )
    }

    #[test]
    #[cfg(feature = "std")]
    fn alphabet() {
        assert_eq!(
            ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --.. .-.-.- --..-- ",

            "ABCDEFGHIJKLMNOPQRSTUVWXYZ.,"
                .to_morse_string(MorseStandard)
                .unwrap()
        )
    }

    #[test]
    #[cfg(feature = "std")]
    fn morse_to_char() {
        for ch in "ABCDEFGHIJKLMNOPQRSTUVWXYZ.,".chars() {
            let mut buffer = [MorseCode::Dot; 30];
            let end = MorseStandard.char_to_morse(&ch, &mut buffer).expect(
                format!(
                    "Failed to convert {:?} to morse",
                    ch
                ).as_str(),
            );
            let morse = &buffer[0..end];
            assert_eq!(
                ch,
                MorseStandard.morse_to_char(&morse).expect(
                    format!("Failed to convert {:?} to {:?}", morse, ch)
                        .as_str(),
                )
            )
        }
    }
}
