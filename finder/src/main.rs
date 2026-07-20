use clap::Parser;
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use regex::Regex;
use std::io::{self, BufRead, Write};
use std::env;

#[derive(Parser)]
#[command(version, long_about = None)]
#[command(about="finds and replaces text in files")]
struct Cli {
    /// the file to use for input
    input: PathBuf,
    
    /// the file to use for output
    output: Option<PathBuf>,

    /// the regular expression to find within the input file
    #[arg(long, short)]
    find: Option<String>,

    /// the regular expression to convert matches for output
    #[arg(long, short)]
    replace: Option<String>,

    /// When set, write non-matched lines to the output file in addition to edited matched lines
    #[arg(long, short)]
    all: bool
}

fn print(opf: &mut Option<io::BufWriter<File>>,
    line: &str) {
        
        println!("{line}");
        if let Some(stream) = opf {
            stream.write(line.as_bytes()).unwrap();
            stream.write(&[10]).unwrap();
    }
}

fn main() {
    let cli = Cli::parse();

    let ipf = OpenOptions::new()
        .read(true)
        .open(cli.input)
        .expect("Cannot open input file");

    let ipf = io::BufReader::new(ipf).lines();

    let mut opf = match cli.output {
        None => None,
        Some(filename) => {
            let opff = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(filename)
                .expect("Cannot open output file");
            Some(io::BufWriter::new(opff))
        }
    };

    let find_expr = cli.find.map_or(None, |expr| {
        Some(Regex::new(&expr)
            .expect("Invalid find expression"))
    });

    let replace = cli.replace.map_or(None, |mut replace| {
        /* include environment variables here */
        replace = env::vars().fold(replace, | acc, (var, val) | {
            acc.replace(&format!(r"$${var}$$"), &val)
        });
        Some(replace)
    });

    for line in ipf {
        match line {
            Err(x) => panic!("Failed to read file: {x}"),

            Ok(line) => match find_expr {
                
                None => print(&mut opf, &line),

                Some(ref re) => {
                    if re.is_match(&line) {

                        match replace {
                            None => {print(&mut opf, &line);}

                            Some(ref replace) => {
                                let s = re.replace_all(&line, replace);
                                print(&mut opf, &s);
                            },
                        }
                    }
                    else {
                        if cli.all {print(&mut opf, &line);}
                    }
                }
            }
        }
    }


}