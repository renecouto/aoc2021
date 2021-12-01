use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut prev: u64 = 0;
    let mut changes: u64 = 0;
    for (i, line) in reader.lines().enumerate() {
        let n: u64 = line?.parse().unwrap();
        if i != 0 && n > prev{
            changes += 1;
        }
        prev = n;
    }
    println!("{}", changes);
    Ok(())
}
