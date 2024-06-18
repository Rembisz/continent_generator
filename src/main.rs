use colored::Colorize;
use rand::Rng;
use std::fs;
use std::io;

struct Position {
    x: isize,
    y: isize,
}

fn random_seed() {
    let mut count = 0;
    let mut seed = String::new();
    while count != 10_000 {
        count += 1;
        let mut rng = rand::thread_rng();
        let rand_data = rng.gen_range(0..100);
        match rand_data {
            0..=24 => seed.push_str("^"),
            25..=49 => seed.push_str(">"),
            50..=74 => seed.push_str("v"),
            75..=100 => seed.push_str("<"),
            _ => println!("Failed to generate seed."),
        };
    }

    println!("Writing...");

    fs::File::create("src/seed.txt").expect("File creation/truncation failure");
    fs::write("src/seed.txt", seed).expect("File write failure");
}

fn print_2d(array: &Vec<Vec<u32>>, size: isize) -> () {
    let mut row: usize = 0;
    let mut count = 0;
    while count != size * size {
        for col in array {
            if count % size == 0 && count != 0 {
                println!(";");
                if row as isize == size - 1 {
                    ();
                } else {
                    row += 1;
                }
            } else {
                if col[row] == 0 {
                    print!("{}", "_  ".blue());
                } else {
                    if col[row] > 9 {
                        print!("{} ", col[row].to_string().red());
                    } else if col[row] > 1 {
                        print!("{}  ", col[row].to_string().yellow());
                    } else {
                        print!("{}  ", col[row].to_string().green());
                    }
                }
            }
            count += 1;
        }
    }
}

fn size(seed: &String) -> usize {
    let mut list = Vec::new();
    let mut x_value: isize = 0;
    let mut y_value: isize = 0;

    for char in seed.trim().chars() {
        match char {
            '^' => {
                y_value += 1;
                list.push(y_value.abs());
            }
            'v' => {
                y_value -= 1;
                list.push(y_value.abs());
            }
            '>' => {
                x_value += 1;
                list.push(x_value.abs());
            }
            '<' => {
                x_value -= 1;
                list.push(x_value.abs());
            }
            _ => (),
        };
    }
    let max_opt = list.iter().max();
    let max = match max_opt {
        Some(max) => *max as usize + 2,
        None => 0,
    };

    if max % 2 == 0 {
        max
    } else {
        max + 1
    }
}

fn navigate(raw_size: usize, seed: &String) -> i32 {
    println!(
        "Array initializing with size {raw_size} * 2 = {}",
        raw_size * 2
    );
    let size = raw_size as isize * 2;
    let mut map: Vec<Vec<u32>> = vec![vec![Default::default(); raw_size * 2]; raw_size * 2];
    let mut location = Position { x: 0, y: 0 };
    map[(size / 2) as usize][(size / 2) as usize] = 1;
    let mut runs = 0;

    for (char_index, char) in seed.trim().chars().enumerate() {
        match char {
            '^' => {
                location.y += 1;
            }
            'v' => {
                location.y -= 1;
            }
            '>' => {
                location.x += 1;
            }
            '<' => {
                location.x -= 1;
            }
            _ => continue,
        };
        let col_index = match usize::try_from(location.x + (size / 2)) {
            Ok(i) => i,
            Err(e) => {
                eprintln!(
                    "Failed to convert col_index = {} to usize with error {e}. run : {}",
                    location.x + (size / 2),
                    runs
                );
                break;
            }
        };
        let col = match map.get_mut(col_index) {
            Some(col) => col,
            None => {
                eprintln!("At character {char_index} attempted to index into columns with index of {col_index} and went out of bounds.");
                break;
            }
        };
        let row_index = match usize::try_from(location.y + (size / 2)) {
            Ok(i) => i,
            Err(e) => {
                eprintln!(
                    "Failed to convert row_index = {} to usize with error {e}. run : {}",
                    location.x + (size / 2),
                    runs
                );
                break;
            }
        };
        match col.get_mut(row_index) {
            Some(row) => *row += 1,
            None => {
                eprintln!("At character {char_index} attempted to index into column at {col_index} with index of {row_index} and went out of bounds.");
                break;
            }
        }
        runs += 1;
    }

    print_2d(&map, size);

    let mut tiles = 0;
    for vector in map.iter() {
        for num in vector.iter() {
            match num {
                0 => (),
                _ => tiles += 1,
            };
        }
    }
    tiles
}

fn main() {
    loop {
        let mut end = false;
        println!("Would you like to generate a new seed?\nType x to end the program.");
        loop {
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read entry.");

            match answer.trim() {
                "y" | "Y" | "yes" | "Yes" | "YES" => {
                    random_seed();
                    break;
                }
                "n" | "N" | "no" | "No" | "NO" => break,
                "x" | "X" => {
                    end = true;
                    break;
                }
                _ => println!("Invalid. Please enter yes or no."),
            }
        }
        if end {
            break;
        } else {
            let seed = match fs::read_to_string("src/seed.txt") {
                Ok(steps) => steps,
                Err(e) => String::from(format!("{}", e)),
            };

            let tiles = navigate(size(&seed), &seed);

            println!("Land tiles: {}", tiles);
        }
    }
}
