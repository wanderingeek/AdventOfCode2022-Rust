use log::info;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // This will hold the total calories carried by each elf.
    // The [vector index + 1] will denote the "name" of the elf.
    // Example:- If we want the number of calories carried by Elf 5, we will get elf_total_calories[4]
    let mut elf_total_calories: Vec<i32> = vec![];
    let mut elf_count = 0;

    // Input file reader
    let input_file_handle = File::open("input.txt").expect("Failed to read input file");
    let input_file_lines = BufReader::new(input_file_handle).lines();

    let mut current_elf_calorie_count = 0;

    env_logger::init();
    info!(" Begin calorie computation!");

    let mut max_calorie_count = 0;

    // Go through each line of the input
    for input_line_result in input_file_lines {
        if let Ok(input_line) = input_line_result {
            // We are counting bytes here. But an empty line should have zero bytes between two newline chars.
            // So a check against zero should work
            if input_line.trim().len() == 0 {
                // This is a blank line.
                // Total calories for the previous elf counted. Time to push this to the vector.
                info!(
                    "Encountered blank line! Total calories = {} for elf {}",
                    current_elf_calorie_count,
                    elf_total_calories.len() + 1
                );
                elf_total_calories.push(current_elf_calorie_count);

                // Check if this is the elf carrying the most calories
                if current_elf_calorie_count > max_calorie_count {
                    max_calorie_count = current_elf_calorie_count;
                }

                // Reset the count so that we can start counting calories for the next elf.
                info!("Resetting calorie counts to zero");
                current_elf_calorie_count = 0;

                elf_count += 1;

                // Read next line from input file
                continue;
            } else {
                // Convert string to integer
                // Turbofish :)
                let current_line_parsed = input_line.parse::<i32>();

                // Check for parsing error
                if let Ok(calories) = current_line_parsed {
                    //Add the calories on the current line to the total calorie count of the elf
                    info!(
                        "Current Calorie Count = {}. Adding {}",
                        current_elf_calorie_count, calories
                    );
                    current_elf_calorie_count += calories
                }
            }
        } else {
            panic!("Failed to read input lines! Exiting program!");
        }
    } //for loop

    // Handle edge case, where the last line of the input is not a blank line.
    // Add total calories for the last elf to the vector
    if current_elf_calorie_count > 0 {
        info!(
            "Current Calorie Count = {}. Adding entry for the last elf",
            current_elf_calorie_count
        );
        elf_total_calories.push(current_elf_calorie_count);
        elf_count += 1;
    }

    println!("Total Elves {}", elf_count);
    println!(
        "The elf carrying the most calories is carrying {} calories",
        max_calorie_count
    );
}