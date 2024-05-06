use std::collections::HashMap;
use std::fs;
use std::io;

fn parse_dataset(contents: &str) -> HashMap<String, i32> {
    contents
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                let move_str = parts[0].to_string();
                let value = parts[1].parse::<i32>().ok()?;
                Some((move_str, value))
            } else {
                None
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let dataset1_contents = fs::read_to_string("mine.txt")?;
    let dataset2_contents = fs::read_to_string("stockfish.txt")?;

    let map1 = parse_dataset(&dataset1_contents);
    let map2 = parse_dataset(&dataset2_contents);

    let mut non_matching_mine_theirs: Vec<[(String, i32); 2]> = Vec::new();

    for (move_str, value) in &map1 {
        if let Some(value2) = map2.get(move_str) {
            if value != value2 {
                non_matching_mine_theirs.push([
                    (move_str.to_string(), *value),
                    (move_str.to_string(), *value2),
                ]);
            }
        } else {
            non_matching_mine_theirs
                .push([(move_str.to_string(), *value), (move_str.to_string(), -1)]);
        }
    }
    let mut non_matching_theirs_mine: Vec<[(String, i32); 2]> = Vec::new();
    for (move_str, value) in &map2 {
        if let Some(value2) = map1.get(move_str) {
            if value != value2 {
                non_matching_theirs_mine.push([
                    (move_str.to_string(), *value),
                    (move_str.to_string(), *value2),
                ]);
            }
        } else {
            non_matching_theirs_mine
                .push([(move_str.to_string(), *value), (move_str.to_string(), -1)]);
        }
    }

    println!(
        "Non-matching values (my results), (their results): {:?}",
        non_matching_mine_theirs
    );
    println!(
        "Non-matching values (their results), (my results): {:?}",
        non_matching_theirs_mine
    );

    Ok(())
}
