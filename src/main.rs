use std::collections::HashMap;
use std::io::{stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = get_file()?;

    let table = create_table(file)?;

    let json = format!("{:?}", table);

    stdout().write(json.as_bytes())?;

    stdout().write("\n".as_bytes())?;

    Ok(())
}

fn create_table(
    file: std::fs::File,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut table = Vec::new();
    let header_row = match rdr.records().next() {
        Some(r) => r?,
        None => return Ok(table),
    };
    let headers: Vec<String> = header_row.iter().map(|h| String::from(h)).collect();
    for line in rdr.records() {
        let mut row: HashMap<String, String> = HashMap::new();
        let line = line?;
        for (i, cell) in line.iter().enumerate() {
            row.insert(headers[i].clone(), String::from(cell));
        }
        table.push(row);
    }

    Ok(table)
}

fn get_file() -> Result<std::fs::File, &'static str> {
    let filepath = match std::env::args().nth(1) {
        Some(f) => f,
        None => return Err("Please enter a filename"),
    };
    match std::fs::File::open(filepath) {
        Ok(f) => return Ok(f),
        Err(_) => return Err("Unable to read file"),
    };
}
