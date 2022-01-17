mod args;
mod lexer;
mod reader;
use lexer::Lexer;
use reader::TranslationUnit;
use std::{env, process};

fn main() {
    let cli_args = args::Args::new(env::args().collect::<Vec<String>>()).unwrap_or_else(|err| {
        eprintln!("mando error: {}", err);
        process::exit(1);
    });

    let index = TranslationUnit::new(cli_args.get_filename()).unwrap_or_else(|err| {
        eprintln!("mando error: {}: {}", err, cli_args.get_filename());
        process::exit(1);
    });
    let _tokenizer = Lexer::new(index.file_content());
}
