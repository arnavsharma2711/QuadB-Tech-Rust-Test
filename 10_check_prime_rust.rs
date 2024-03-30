// Check if a number is prime in Rust

fn is_prime(candidate_number: u64) -> bool {
    if candidate_number <= 2 {
        return candidate_number == 2;
    }

    if candidate_number % 2 == 0 || candidate_number % 3 == 0 {
        return false;
    }

    let limit = (candidate_number as f64).sqrt() as u64 + 1;
    for divisor in (5..limit).step_by(6) {
        if candidate_number % divisor == 0 || candidate_number % (divisor + 2) == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Would you like to use a dummy number? \n 1 => Prime Number \n 2 => Non-Prime Number \n 3 => Custom Number");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let selected_number = if user_selection.trim() == "1" {
        29
    } else if user_selection.trim() == "2" {
        40
    } else if user_selection.trim() == "3" {
        println!("Enter a number: ");
        let mut user_number_input = String::new();
        std::io::stdin().read_line(&mut user_number_input).expect("Failed to read line");
        user_number_input.trim().parse::<u64>().expect("Invalid number input")
    } else {
        println!("Invalid input");
        return;
    };

    if is_prime(selected_number) {
        println!("{} is a prime number!", selected_number);
    } else {
        println!("{} is not a prime number.", selected_number);
    }
}
