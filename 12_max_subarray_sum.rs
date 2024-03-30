// Find the maximum sub_array sum in rust

fn max_sub_array_sum(input_array: &[i32]) -> i32 {
    let mut sum = 0;
    let mut max_sum = input_array[0];

    for num in input_array {
        sum += num;
        max_sum = max_sum.max(sum);

        if sum < 0 {
            sum = 0;
        }
    }
    max_sum
}


fn main() {
    println!("Would you like to use a dummy array? \n 1 => Dummy Array \n 2 => Custom Array");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let input_numbers: Vec<i32> = if user_selection.trim() == "1" {
        vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]
    } else if user_selection.trim() == "2" {
        println!("Enter the numbers in the array separated by spaces: ");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect()
    } else {
        println!("Invalid input");
        return;
    };

    println!("The chosen array is: {:?}", input_numbers);

    let max_sum = max_sub_array_sum(&input_numbers);
    println!("The maximum sub_array sum of the array is: {}", max_sum);
}
