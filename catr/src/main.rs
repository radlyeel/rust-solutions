// alias for https://docs.rs/anyhow/latest/anyhow/type.Result.html
use anyhow::Result;
// Easy access to CLI args.
use clap::Parser;

// File ops
use std::fs::File;

// File objects
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of 'cat'
struct Args {

    /// list of requested files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    ///  Number lines
    #[arg(short('n'), 
        long("number"),
        conflicts_with("number_nonblank_lines"))]
    number_lines: bool, 

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> { 
    match filename {
        // the - filename means use stdin
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), 
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), 
    }
}

#[allow(unused_variables)] 
fn run(args: Args) -> Result<()> { 
    let mut linecount:i32;
    for filename in args.files {
        // Restart line counter on every file
        linecount = 0;
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => 
                for line in file.lines() {
                    let line = line.expect("Can't read line");
                    if args.number_lines || (args.number_nonblank_lines && line != "") {
                        linecount += 1;
                    }
                    if args.number_lines || (args.number_nonblank_lines && line != "") {
                        println!("{:6}\t{}", linecount, line);
                    } else {
                        println!("{}", line);
                    }
                },
        }
    } 
    Ok(()) 
}


fn main() {
    let _ = run(Args::parse());
}
