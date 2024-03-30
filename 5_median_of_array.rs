// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn find_median(sorted_numbers: &[i32]) -> f64 {
    let mid_index = sorted_numbers.len() / 2;

    // Handle even and odd lengths differently
    if sorted_numbers.len() % 2 == 0 {
        let left_of_middle = sorted_numbers[mid_index - 1];
        let right_of_middle = sorted_numbers[mid_index];
        (left_of_middle + right_of_middle) as f64 / 2.0
    } else {
        sorted_numbers[mid_index] as f64
    }
}

fn main() {
    println!("Would you like to use a dummy array? \n 1 => Dummy Array \n 2 => Custom Array");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let selected_array: Vec<i32> = if user_selection.trim() == "1" {
        vec![1, 2, 3, 3, 4, 4, 5]
    } else if user_selection.trim() == "2" {
        println!("Enter the numbers in the array separated by spaces: ");
        let mut user_array_input = String::new();
        std::io::stdin().read_line(&mut user_array_input).expect("Failed to read line");
        user_array_input
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect()
    } else {
        println!("Invalid input");
        return;
    };

    println!("The chosen array is: {:?}", selected_array);

    let median_value = find_median(&selected_array);
    println!("The median of the array is: {}", median_value);
}