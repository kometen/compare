// https://github.com/clap-rs/clap/blob/master/examples/07_option_args.rs

extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let mut filename_vector = Vec::new();

    // Command line parameters.
    let matches = App::new("compare")
        .version("0.1")
        .about("compare sets")
        .author("Claus Guttesen")
        .arg(Arg::with_name("file")
            .help("input filename")
            .required(true)
            .takes_value(true)
            .short("f")
            .long("filename")
            .multiple(true)
        )
        .arg(Arg::with_name("delimiter")
            .help("delimiter")
            .required(true)
            .takes_value(true)
            .short("d")
            .multiple(false)
        )
        .arg(Arg::with_name("column")
            .help("column (zero-based indice)")
            .required(true)
            .takes_value(true)
            .short("c")
            .multiple(false)
        )
        .get_matches();
    
    // Put filename(s) in vector.
    if let Some(in_v) = matches.values_of("file") {
        for in_filename in in_v {
            filename_vector.push(in_filename);
        }
    }

    let delimiter = matches.value_of("delimiter").unwrap();

    let column = match matches.value_of("column").unwrap().trim().parse::<usize>() {
        Ok(column) => column,
        Err(_e) => { 0 }
    };

    for filename in filename_vector {
        println!("Input file: {}", filename);
        let _result = split_lines(filename.to_string(), delimiter.to_string(), column);
    }

    Ok(())
}

// https://www.tutorialspoint.com/rust/rust_string.htm
fn split_lines(f: String, d: String, c: usize) -> Result<(), Error> {
    println!("delimiter: {}", d);
    println!("column: {:?}", c);
    let _f = f.clone();
    let input_file = File::open(f);
    let _ = match input_file {
        Ok(input_file) => {
            let reader = BufReader::new(input_file);
            for line in reader.lines() {
                let s = line.unwrap().to_string();
                let tokens:Vec<&str> =  s.split(&d).collect();
                if tokens.len() > c {
                    println!("{}", tokens[c]);
                }
            }
        },
        Err(_) => {
            println!("file not found: {}", _f)
        }
    };

    Ok(())
}