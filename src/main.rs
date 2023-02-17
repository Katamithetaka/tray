mod executer;
mod extensions;
mod lexer;
mod parser;
mod reader;
use std::env;

use extensions::IteratorExt;

fn main() {
    let args = env::args().collect_into_vec();
    if let Some(file_name) = args.get(1) {
        let content = std::fs::read_to_string(file_name)
            .expect(format!("Couldn't read file `{file_name}`. Make sure the file is in the right directory and make sure the file is readable.").as_str());

        let _iterator = content.lines();

        return;
    }

    let mut lines = reader::LineReader {};
    loop {
        let line = lines.next();
        if let Some(line) = line {
            let tokens = lexer::parse_one(line.clone());
            if let Ok(tokens) = tokens {
                if let Some(expression) = parser::parse(&tokens) {
                    dbg!(&expression);
                    dbg!(executer::execute(&expression));
                }
            } else if let Err(err) = tokens {
                err.arrow_error(line);
            }
        }
    }
}
