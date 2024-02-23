use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

fn traverse_deque(deque: &VecDeque<&str>) {
    for element in deque {
        println!("{}", element);
    }
}

fn random_string(length: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let mut rng = thread_rng();
    let random_chars: Vec<char> = chars.choose_multiple(&mut rng, length).cloned().collect();

    random_chars.into_iter().collect()
}

fn genRandomToDeque(deque: &mut VecDeque<&str>, size: usize) {
    for _ in 0..size {
        let random_length = thread_rng().gen_range(1..2);
        let random_word = random_string(random_length);
        deque.push_back(Box::leak(random_word.into_boxed_str()));
    }
}

fn main() {
    // Define VecDeque object with string data type
    let mut deque: VecDeque<&str> = VecDeque::new();

    genRandomToDeque(&mut deque, 10);

    traverse_deque(&deque);

    println!();
    println!("Should be A,B,C,D");
    while !deque.is_empty() {
        print!("{},", deque.front().unwrap());
        deque.pop_front();
    }
    println!();

    deque.push_back("A");
    deque.push_back("B");
    deque.push_back("C");
    deque.push_back("D");
    println!();
    println!("Should be D,C,B,A");
    while !deque.is_empty() {
        print!("{},", deque.back().unwrap());
        deque.pop_back();
    }
    println!();
}

#[cfg(test)]
pub mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_vecdeque() {
        let mut deque: VecDeque<u32> = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque.push_back(4);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(4));
        assert_eq!(deque.pop_front(), None);

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);
        deque.push_front(4);

        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(4));
        assert_eq!(deque.pop_back(), None);
    }
}
