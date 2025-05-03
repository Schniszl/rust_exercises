use std::fs;
use std::io::{self, Write};
use std::path::{Path};
use minesweeper::annotate;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    for file_path in &args[1..] {
        let path = Path::new(file_path);
        // check if pfad exist
        let content = fs::read_to_string(path)?;
        let input: Vec<&str> = content.lines().collect();

        // check result of annotate
        let result = annotate(&input).map_err(|e| {
            // eprintln!("Fehler in {}: {}", file_path, e);
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;

        let output_path = path.with_extension("out");
        write_output_file(&output_path, &result)?;  // jetzt korrekt

        // println!("Output geschrieben: {}", output_path.display());
    }

    Ok(())
}
fn write_output_file(path: &Path, lines: &[String]) -> io::Result<()> {
    // check for path to created
    let mut file = fs::File::create(path)?;
    for line in lines {
        // check for space
        writeln!(file, "{}", line)?;
    }
    Ok(())
}


