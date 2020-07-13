use std::env;

#[test]
fn check_mergesort() {
    let letters = vec!["s", "a", "m", "p", "l", "e"];
    let shouldbe = vec!["a", "e", "l", "m", "p", "s"];
    let sorted = mergesort(letters);
    assert_eq!(sorted, shouldbe);

    let words = vec![
        "now", "is", "the", "time", "for", "all", "good", "men", "to", "come", "to", "the", "aid",
        "of", "their", "country",
    ];
    let shouldbe = vec![
        "aid", "all", "come", "country", "for", "good", "is", "men", "now", "of", "the", "the",
        "their", "time", "to", "to",
    ];
    let sorted = mergesort(words);
    assert_eq!(sorted, shouldbe);
}

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

    let words: Vec<&str> = args.iter().skip(1).map(AsRef::as_ref).collect();
    println!("Arrived: {:?}", words);

    let words = mergesort(words);
    println!("Sorted: {:?}", words);
}

/// Mergesort
/// Section 1.4, page 27
pub fn mergesort(words: Vec<&str>) -> Vec<&str> {
    if words.len() < 2 {
        return words;
    }

    let midpoint = (words.len() + 1) / 2;
    let mut left: Vec<&str> = Vec::new();
    left.extend_from_slice(&words[..midpoint]);
    let left = mergesort(left);

    let mut right: Vec<&str> = Vec::new();
    right.extend_from_slice(&words[midpoint..]);
    let right = mergesort(right);

    let mut words = left;
    words.extend(right);

    let words = merge(words, midpoint);
    return words;
}

pub fn merge(words: Vec<&str>, midpoint: usize) -> Vec<&str> {
    let mut left_index = 0;
    let mut right_index = midpoint;
    let size = words.len();
    let mut sorted: Vec<&str> = Vec::new();

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
