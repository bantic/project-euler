use std::io::{File};

fn char_value(char:char) -> Option<uint> {
    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut idx = 0;
    for c in alpha.chars() {
        idx = idx + 1;
        if c == char {
            return Some(idx);
        }
    }
    None
}

fn value(str:&str) -> uint {
    let mut sum = 0u;
    for char in str.chars() {
        sum += char_value(char).unwrap();
    }
    sum
}

fn main() {
    let collection = File::open(&Path::new("p022_names.txt")).read_to_string();
    let string = match collection {
        Ok(s) => s,
        Err(e) => fail!("error {}",e)
    };
    let mut names = vec![];
    let chars_to_trim: &[_] = ['"'];
    for name in string.as_slice().split(',') {
        let trimmed = name.trim_chars(chars_to_trim);
        names.push(trimmed);
    }
    names.sort();

    let mut sum = 0u;
    let mut name_idx = 1u;
    for name in names.iter() {
        sum += name_idx * value(name.as_slice());
        name_idx += 1;
    }

    println!("sum {}",sum);
}
