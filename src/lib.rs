//! wordcount is simple count of chars or words or lines
//! see [`count`](fn.count.html)

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// use option for [`count`](fn.count.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// count of chars
    Char,
    /// count of words
    Word,
    /// count of lines
    Line,
}

/// option default value
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// count of chars or words  or lines, read from input. they must encode by UTF-8
/// 
/// count target is controllable by options
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): a char by Unicode
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): regex "\w+"
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): "\n" or "\r\n"
///
/// # Examples
/// for example, count of word
///
/// ```
/// use std::io::Cursor;
/// use wordcount::{count, CountOption};
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
///
/// # Panics
///
/// input file encoding is not UTF-8
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => {
                *freqs.entry(line.to_string()).or_insert(0) += 1;
            }
        }
    }
    freqs
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn word_count_works() {
        use std::io::Cursor;

        let mut exp = HashMap::new();
        exp.insert("aa".to_string(), 1);
        exp.insert("bb".to_string(), 2);
        exp.insert("cc".to_string(), 1);

        assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
    }

    //#[test]
    //fn word_count_fails() {
    //    use std::io::Cursor;
    //    let mut exp = HashMap::new();
    //    exp.insert("aa".to_string(), 1);
    //    assert_eq!(count(Cursor::new("aa  cc bb"), CountOption::Word), exp);
    //}

    #[test]
    fn result_test() -> std::io::Result<()> {
        use std::fs::{read_to_string, remove_file, write};
        write("test2.txt", "message")?;
        let message = read_to_string("test2.txt")?;
        remove_file("test2.txt")?;
        assert_eq!(message, "message");
        Ok(())
    }

    #[test]
    #[should_panic]
    fn word_count_o_not_contain_unknown_words() {
        use std::io::Cursor;

        count(
            Cursor::new([
                b'a', // a
                0xf9, 0x90, 0x80,
                0xe3, 0x81, 0x82, 
            ]),
            CountOption::Word,
        );
    }

    macro_rules! assert_map {
        ($expr: expr, {$($key: expr => $value: expr),*}) => {
            $(assert_eq!($expr[$key], $value));*
        };
    }
    #[test]
    fn word_count_works3() {
        use std::io::Cursor;
        let freqs = count(Cursor::new("aa cc dd cc"), CountOption::Word);

        assert_eq!(freqs.len(), 3);
        assert_map!(freqs, {"aa" => 1, "cc" => 2, "dd" => 1});
    }

}
