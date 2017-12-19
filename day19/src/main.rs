extern crate day19;
use day19::collect_letters;

extern crate util;
use util::file_as_by;

fn main() {
    let mut maze = file_as_by::<char, _>("input.txt", |line| {
        line.char_indices()
            .filter(|&(_, c)| c == ' ' || !c.is_whitespace())
            .map(|(idx, c)| &line[idx..(idx + c.len_utf8())])
            .collect()
    }).expect("problem reading or parsing input");
    if let Some(max_width) = maze.iter().map(|row| row.len()).max() {
        for row in &mut maze {
            row.resize(max_width, ' ');
        }
    }
    let (letters, steps) = collect_letters(&maze).expect("should be able to run maze");
    println!("letters found: {}", letters.iter().collect::<String>());
    println!("steps taken: {}", steps);
}
