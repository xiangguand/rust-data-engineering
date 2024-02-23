use std::collections::LinkedList;

fn traverse_linklist<T: std::fmt::Display>(list: &LinkedList<T>) {
    print!("List: ");
    for item in list {
        print!("{}, ", item);
    }
    println!("");
}

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    
    list.push_back('a' as u32);
    list.push_back('b' as u32);
    
    println!("Should be 97, 98");
    traverse_linklist(&list);
}

#[cfg(test)]
mod tests {

    use std::collections::LinkedList;
    #[test]
    fn test_linklist_ordering() {
        let mut list: LinkedList<u32> = LinkedList::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }
}
