// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn find_first_occurrence(numbers: &[i32], target_number: i32) -> Option<usize> {
    let mut lower_bound = 0;
    let mut upper_bound = numbers.len() - 1;

    while lower_bound <= upper_bound {
        let middle_index = lower_bound + (upper_bound - lower_bound) / 2;

        if numbers[middle_index] == target_number {
            if middle_index == 0 || numbers[middle_index - 1] < target_number {
                return Some(middle_index);
            } else {
                upper_bound = middle_index - 1;
            }
        } else if numbers[middle_index] < target_number {
            lower_bound = middle_index + 1;
        } else {
            upper_bound = middle_index - 1;
        }
    }

    None
}

fn main() {
    println!("Would you like to use a dummy array? \n 1 => Dummy Array \n 2 => Custom Array");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let selected_array: Vec<i32> = if user_selection.trim() == "1" {
        vec![1, 2, 3, 3, 4, 4, 5]
    } else if user_selection.trim() == "2" {
        println!("Enter the numbers in the array separated by spaces: ");
        let mut custom_array_input = String::new();
        std::io::stdin().read_line(&mut custom_array_input).expect("Failed to read line");
        custom_array_input
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect()
    } else {
        println!("Invalid input");
        return;
    };

    println!("The chosen array is: {:?}", selected_array);

    println!("Enter the number to search for: ");
    let mut target_number_input = String::new();
    std::io::stdin().read_line(&mut target_number_input).expect("Failed to read line");
    let target_number = target_number_input.trim().parse::<i32>().expect("Invalid number");

    if let Some(index) = find_first_occurrence(&selected_array, target_number) {
        println!("The first occurrence of {} is at index {}.", target_number, index);
    } else {
        println!("{} is not found in the array.", target_number);
    }
}