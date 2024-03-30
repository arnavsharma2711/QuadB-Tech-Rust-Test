// Implement a function that checks whether a given string is a palindrome or not.

fn is_palindrome(input_text: &str) -> bool {
    let lowercased_text = input_text.to_lowercase();
    let characters: Vec<char> = lowercased_text.chars().collect();

    for index in 0..(characters.len() / 2) {
        if characters[index] != characters[characters.len() - 1 - index] {
            return false;
        }
    }

    true
}

fn main() {
    println!("Would you like to use a dummy string? \n 1 => Palindrome String \n 2 => None palindrome String \n 3 => Custom String");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let chosen_text = if user_selection.trim() == "1" {
        "kayak".to_string()
    } else if user_selection.trim() == "2" {
        "hello".to_string()
    } else if user_selection.trim() == "3" {
        println!("Enter a string: ");
        let mut custom_input = String::new();
        std::io::stdin().read_line(&mut custom_input).expect("Failed to read line");
        custom_input.trim().to_string()
    } else {
        println!("Invalid input");
        return;
    };

    if is_palindrome(&chosen_text) {
        println!("{} is a palindrome!", chosen_text);
    } else {
        println!("{} is not a palindrome.", chosen_text);
    }
}
