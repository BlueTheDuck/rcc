use std::path::PathBuf;

use clap::Parser;
use rcc::{ast::parse_stream};

#[derive(Parser)]
struct Args {
    path: PathBuf,

    #[clap(short, long, default_value = "a.out")]
    output: Option<PathBuf>,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    assemble: bool,

    #[clap(short, long)]
    compile: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = std::fs::File::open(args.path)?;

    let content = unsafe { memmap2::Mmap::map(&file)? };
    let content = std::str::from_utf8(&content)?;

    let parsed = rcc::lexer::parsers::parse_program(&content);
    let stream = rcc::lexer::stream::TokenStream::new(&parsed);

    if true || args.verbose {
        for token in &parsed {
            print!("{} ", token);
        }
        println!();
    }

    let program = parse_stream(stream);
    for node in program {
        println!("{node} ");
    }

    Ok(())
}
