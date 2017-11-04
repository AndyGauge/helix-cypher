#[macro_use]
extern crate helix;
extern crate rayon;
use rayon::prelude::*;

ruby! {
    class TextTransform {
        def rot13(inp: String) -> String {
            let chars: Vec<char> = inp.chars().collect();
            chars.into_par_iter().map(|c| {
                match c {
                    'A' ... 'M' | 'a' ... 'm' => ((c as u8) + 13) as char,
                    'N' ... 'Z' | 'n' ... 'z' => ((c as u8) - 13) as char,
                    _ => c
                }
            }).collect()
        }
    }
}
