// Implement a function that finds the longest common prefix of a given set of strings.

fn find_longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut common_prefix = String::new();

    let minimum_string_length = strings.iter().map(|s| s.len()).min().unwrap();

    for index in 0..minimum_string_length {
        let current_character = strings[0].chars().nth(index).unwrap();

        if strings.iter().all(|s| s.chars().nth(index) == Some(current_character)) {
            common_prefix.push(current_character);
        } else {
            break;
        }
    }

    common_prefix
}

fn main() {
    println!("Would you like to use a dummy string? \n 1 => Dummy String \n 2 => Custom String");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let chosen_string: String = if user_selection.trim() == "1" {
        "flower flow flight".to_string() // This string is better for testing purposes as it has a common prefix
    } else if user_selection.trim() == "2" {
        println!("Enter the string: ");
        let mut user_string_input = String::new();
        std::io::stdin().read_line(&mut user_string_input).expect("Failed to read line");
        user_string_input
    } else {
        println!("Invalid input");
        return;
    };

    println!("The chosen string is: {:?}", chosen_string);

    let trimmed_input = chosen_string.trim();
    let words: Vec<&str> = trimmed_input.split_whitespace().collect();

    println!("The longest common prefix is: {}", find_longest_common_prefix(&words));
}
