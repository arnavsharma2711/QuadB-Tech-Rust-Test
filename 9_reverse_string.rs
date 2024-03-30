// Reverse a string in Rust

fn reverse_string(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn main() {
    println!("Would you like to use a dummy string? \n 1 => Dummy String \n 2 => Custom String");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let input_string = if user_selection.trim() == "1" {
        "abcdef 12345".to_string()
    } else if user_selection.trim() == "2" {
        println!("Enter a string: ");
        let mut custom_input = String::new();
        std::io::stdin().read_line(&mut custom_input).expect("Failed to read line");
        custom_input.trim().to_string()
    } else {
        println!("Invalid input");
        return;
    };

    println!("The chosen string is: {:?}", input_string);

    let reversed_string = reverse_string(&input_string);
    println!("The reversed string is: {}", reversed_string);
}
