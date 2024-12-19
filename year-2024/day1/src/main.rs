use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let file_path = "./input.txt"; // Path to your input file
    match calculate_file_distance(file_path) {
        Ok(compare_list) => println!("Total Distance: {}", compare_list),
        Err(e) => eprintln!("Error: {}", e),
    }
}
// Part 1
// Step 1: `calculate_file_distance`
fn calculate_file_distance(file_path: &str) -> Result<i64, io::Error> {
    let (first_list, second_list) = convert_to_sorted_list(file_path)?;
    Ok(compare_list(&(first_list, second_list)))
}

// Step 2: `convert_to_sorted_list`
fn convert_to_sorted_list(path: &str) -> Result<(Vec<i64>, Vec<i64>), io::Error> {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    // Open the file and read its contents line by line
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // Split by triple space
        let values: Vec<&str> = line.split("   ").collect();

        if values.len() == 2 {
            // Parse and add to respective lists
            if let Ok(first_val) = values[0].trim().parse::<i64>() {
                first_list.push(first_val);
            }
            if let Ok(second_val) = values[1].trim().parse::<i64>() {
                second_list.push(second_val);
            }
        }
    }

    // Sort both lists
    first_list.sort_unstable();
    second_list.sort_unstable();

    Ok((first_list, second_list))
}

// Step 3: `get_total_distance`
fn get_total_distance(tup: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (first_list, second_list) = tup;
    let mut total_sum = 0;

    for (val1, val2) in first_list.iter().zip(second_list.iter()) {
        total_sum += (val2 - val1).abs();
    }

    total_sum
}


//Part 2
fn compare_list( tup: &(Vec<i64>, Vec<i64>)) -> i64{
    let (first_list, second_list) = tup;
    
    let mut score = 0;

    for each_item_2 in second_list{

        let mut compare = 0;

        for each_item_1 in first_list{

            if each_item_1 == each_item_2 {
                compare += 1;

            }
            
      
        }
        score += compare * each_item_2;
    }
    
    score



}