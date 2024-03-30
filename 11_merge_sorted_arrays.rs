// Merge two sorted arrays in Rust

fn merge(first_array: &[i32], second_array: &[i32]) -> Vec<i32> {
    let mut merged_array = Vec::new();
    let mut first_array_index = 0;
    let mut second_array_index = 0;
    while first_array_index < first_array.len() && second_array_index < second_array.len() {
      if first_array[first_array_index] < second_array[second_array_index] {
        merged_array.push(first_array[first_array_index]);
        first_array_index += 1;
      } else {
        merged_array.push(second_array[second_array_index]);
        second_array_index += 1;
      }
    }

    merged_array.extend_from_slice(&first_array[first_array_index..]);
    merged_array.extend_from_slice(&second_array[second_array_index..]);
  
    merged_array
}


fn main() {
    println!("Would you like to use dummy arrays? \n 1 => Dummy Arrays \n 2 => Custom Arrays");
    let mut user_selection = String::new();
    std::io::stdin().read_line(&mut user_selection).expect("Failed to read line");

    let (first_array, second_array) = if user_selection.trim() == "1" {
        (vec![2, 4, 6, 8, 10], vec![1, 3, 4, 5, 7, 9])
    } else if user_selection.trim() == "2" {
        let mut first_array_input = String::new();
        let mut second_array_input = String::new();

        println!("Enter the numbers in the first array separated by spaces: ");
        std::io::stdin().read_line(&mut first_array_input).expect("Failed to read line");
        let array1 = first_array_input
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect();

        println!("Enter the numbers in the second array separated by spaces: ");
        std::io::stdin().read_line(&mut second_array_input).expect("Failed to read line");
        let array2 = second_array_input
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Invalid number"))
            .collect();

        (array1, array2)
    } else {
        println!("Invalid input");
        return;
    };

    println!("The chosen arrays are: {:?} and {:?}", first_array, second_array);

    let merged_array = merge(&first_array, &second_array);
    println!("The merged array is: {:?}", merged_array);
}
