use std::env;

/// Adapted from reading "Algorithms" by Jeff Erickson
/// freely available on http://jeffe.cs.illinois.edu/teaching/algorithms/
pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage: {} word word [word...] (two or more words to sort)",
            args[0]
        );
        return;
    }

    let mut words: Vec<String> = Vec::with_capacity(args.len());
    words.extend_from_slice(&args[1..]);
    println!("Arrived: {:?}", words);

    let words: Vec<String> = mergesort(words);
    println!("Sorted: {:?}", words);
}

/// Mergesort
/// Section 1.4, page 27
pub fn mergesort(words: Vec<String>) -> Vec<String> {
    if words.len() < 2 {
        return words;
    }

    let midpoint = (words.len() + 1) / 2;
    let mut left: Vec<String> = Vec::with_capacity(midpoint);
    left.extend_from_slice(&words[..midpoint]);
    let left = mergesort(left);

    let mut right: Vec<String> = Vec::with_capacity(words.len() - midpoint);
    right.extend_from_slice(&words[midpoint..]);
    let right = mergesort(right);

    let mut words = left;
    words.extend(right);

    let words = merge(words, midpoint);
    return words;
}

pub fn merge(words: Vec<String>, midpoint: usize) -> Vec<String> {
    let mut left_index = 0;
    let mut right_index = midpoint;
    let size = words.len();
    let mut sorted: Vec<String> = Vec::with_capacity(size);

    for _ in 0..size {
        if right_index >= size {
            sorted.push(words[left_index].clone());
            left_index += 1;
        } else if left_index >= midpoint {
            sorted.push(words[right_index].clone());
            right_index += 1;
        } else if words[left_index] < words[right_index] {
            sorted.push(words[left_index].clone());
            left_index += 1;
        } else {
            sorted.push(words[right_index].clone());
            right_index += 1;
        }
    }
    return sorted;
}
