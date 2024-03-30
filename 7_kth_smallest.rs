use std::collections::BinaryHeap;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut heap = BinaryHeap::with_capacity(k);

    for num in arr {
        if heap.len() < k {
            heap.push(*num);
        } else if *num < *heap.peek().unwrap() {
            heap.pop();
            heap.push(*num);
        }
    }

    Some(*heap.peek().unwrap())
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

    println!("Enter the kth smallest number to find: ");
    let mut kth_number_input = String::new();
    std::io::stdin().read_line(&mut kth_number_input).expect("Failed to read line");
    let kth_number = kth_number_input.trim().parse::<usize>().expect("Invalid number");

    if let Some(number) = kth_smallest(&selected_array, kth_number) {
        println!("The {}th smallest number in the array is {}.", kth_number, number);
    } else {
        println!("There is no {}th smallest number in the array.", kth_number);
    }
}