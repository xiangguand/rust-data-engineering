use std::collections::HashMap;

fn main() {
    let test_pattern = vec![1, 2, 5, 1, 3, 5, 3, 1, 6, 9, 1, 8];
    let mut freq_count = HashMap::new();
    for n in test_pattern {
        let freq: &mut i32 = freq_count.entry(n).or_insert(0);
        *freq += 1
    }

    for it in freq_count {
        println!("numbers {}: {}", it.0, it.1);
    }
    println!();
}


#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hashmap() {
        let mut hm = HashMap::<i32, i32>::new();
        hm.insert(1, 2);
        hm.insert(5, 3);
        assert_eq!(hm.get(&1), Some(&2));
        assert_eq!(hm.get(&5), Some(&3));

        let mut hms = HashMap::<String, i32>::new();
        hms.insert("a".to_string(), 2);
        hms.insert("Danny".to_string(), 50);
        assert_eq!(hms.get(&"a".to_string()), Some(&2));
        assert_eq!(hms.get(&"Danny".to_string()), Some(&50));
    }
}
