// Given a string of words, implement a function that returns the shortest word in the string.

fn find_shortest_word(input_text: &str) -> Option<String> {
    let mut shortest_word: Option<String> = None;
    let mut shortest_length = i32::MAX;

    for word in input_text.split_whitespace() {
        let word_length = word.len() as i32;

        if word_length < shortest_length {
            shortest_length = word_length;
            shortest_word = Some(word.to_string());
        }
    }

    shortest_word
}

fn main() {
    println!("Would you like to use a dummy string? \n 1 => Dummy String \n 2 => Custom String");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let chosen_string: String = if user_selection.trim() == "1" {
        "This is a dummy string for testing purposes".to_string()
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

    let trimmed_text = chosen_string.trim();

    if let Some(shortest_word) = find_shortest_word(trimmed_text) {
        println!("The shortest word is: {}", shortest_word);
    } else {
        println!("The string is empty or contains no words.");
    }
}
