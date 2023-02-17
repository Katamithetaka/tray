use std::io::Write;

pub struct LineReader {}
impl Iterator for LineReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        print!("tray> ");
        std::io::stdout().flush().ok()?;
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok()?;
        buffer = buffer.trim_end_matches(['\r', '\n']).to_string();
        return Some(buffer);
    }
}
