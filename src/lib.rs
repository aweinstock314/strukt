#![crate_type = "proc-macro"]
#![feature(proc_macro)]

extern crate proc_macro;
extern crate regex;

use proc_macro::TokenStream;
use regex::Regex;

fn symbol_size(c: char) -> Option<usize> {
    match c {
        'x' | 'c' | 'b' | 'B' => Some(1),
        'h' | 'H' => Some(2),
        'i' | 'I' | 'l' | 'L' | 'f' => Some(4),
        'q' | 'Q' | 'd' => Some(8),
        's' | 'p' => None, // variable-length
        'P' => unimplemented!(), // TODO: introspect target pointer-size?
        _ => unreachable!(),
    }
}

fn symbol_write_method(c: char) -> Option<&'static str> {
    match c {
        'c' | 'b' | 'B' => None, // byteorder isn't needed for single bytes
        'h' => Some("write_i16"),
        'H' => Some("write_u16"),
        'i' | 'l' => Some("write_i32"),
        'I' | 'L' => Some("write_u32"),
        'q' => Some("write_i64"),
        'Q' => Some("write_u64"),
        _ => unimplemented!(), // TODO: float conversions, variable sized things
    }
}

#[proc_macro]
pub fn pack(input: TokenStream) -> TokenStream {
    //let e = syn::parse_expr(format!("({})", input.to_string()));
    let input = input.to_string();
    let re = Regex::new("\"([<>]?)([xcbB\\?hHiIlLqQfdspP]*)\"").unwrap();
    let caps = re.captures(&input).expect("not a valid struct specifier");
    let bo = match &caps[1] {
        "<" | "" => "::byteorder::LittleEndian",
        ">" => "::byteorder::BigEndian",
        _ => unreachable!(),
    };
    let mut output = String::new();
    let num_args = caps[2].len(); // TODO: support run-length encoding as an extension?
    output += "(|";
    assert!(num_args > 0);
    for i in 0..num_args {
        output += &format!("arg{}, ", i);
    }
    output += "| {\n";
    output += "use byteorder::ByteOrder;\n";
    output += "let mut buf = vec![];\n";
    for (i, c) in caps[2].chars().enumerate() {
        if c == 'x' { continue; } // padding
        match (symbol_write_method(c), symbol_size(c)) {
            (Some(method), Some(size)) => {
                output += &format!("buf.extend_from_slice(&{{ let mut tmp = [0u8; {}]; {}::{}(&mut tmp, arg{}); tmp }});\n", size, bo, method, i);
            },
            (None, Some(1)) => {
                output += &format!("buf.extend_from_slice(&[arg{} as u8; 1]);\n", i);
            },
            _ => {
                unimplemented!(); // variable-length s|p case
            }
        }
    }
    output += "\nbuf\n})";
    //println!("output: {}", output);
    output.parse().unwrap()
}
