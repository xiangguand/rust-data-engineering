use std::{borrow::BorrowMut, collections::VecDeque, iter::Successors};

#[derive(Clone)]
struct BinarySearch {
    val: i32,
    parent: Option<Box<BinarySearch>>,
    left: Option<Box<BinarySearch>>,
    right: Option<Box<BinarySearch>>,
}

impl BinarySearch {
    fn new(val: i32) -> BinarySearch {
        BinarySearch {
            val,
            parent: None,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, val: i32) {
        if val < self.val {
            if let Some(ref mut left_node) = self.left {
                left_node.insert(val);
            } else {
                let mut new_node = Box::new(BinarySearch::new(val));
                new_node.parent = Some(Box::new(self.clone()));
                self.left = Some(new_node);
            }
        } else {
            if let Some(ref mut right_node) = self.right {
                right_node.insert(val);
            } else {
                let mut new_node = Box::new(BinarySearch::new(val));
                new_node.parent = Some(Box::new(self.clone()));
                self.right = Some(new_node);
            }
        }
    }
    fn search(&self, val: i32) -> Option<&BinarySearch> {
        if self.val < val {
            if self.right.is_some() {
                return self.right.as_ref().unwrap().search(val);
            }
            return None;
        }
        else if self.val > val {
            if self.left.is_some() {
                return self.left.as_ref().unwrap().search(val);
            }
            return None;
        }

        Some(self)
    }
    fn delete_help(&mut self, val: i32) {
        if self.val > val {
            if let Some(ref mut left_node) = self.left {
                left_node.delete_help(val);
            }
        } else if self.val < val {
            if let Some(ref mut right_node) = self.right {
                right_node.delete_help(val);
            }
        } else {
            unimplemented!()
        }
    }
    fn delete(&mut self, val: i32) -> i32 {
        let dn = self.search(val);
        if dn.is_none() {
            return -1;
        }
        
        self.delete_help(val);
        0
    }
    fn depth_help(&self, depth: i32) -> i32 {
        let mut left_depth = depth;
        let mut right_depth = depth;

        if self.left.is_some() {
            left_depth = self.left.as_ref().unwrap().depth_help(depth + 1);
        }
        if self.right.is_some() {
            right_depth = self.right.as_ref().unwrap().depth_help(depth + 1);
        }

        if left_depth > right_depth {
            left_depth
        } else {
            right_depth
        }
    }
    fn depth(&self) -> i32 {
        self.depth_help(1)
    }
    fn min(&self) -> &BinarySearch {
        if self.left.is_some() {
            self.left.as_ref().unwrap().min()
        } else {
            self
        }
    }
    fn min_mut(&mut self) -> &mut BinarySearch {
        if let Some(ref mut left_node) = self.left {
            left_node.min_mut()
        } else {
            self
        }
    }
    fn max(&self) -> &BinarySearch {
        if self.right.is_some() {
            self.right.as_ref().unwrap().max()
        } else {
            self
        }
    }
    fn print_tree(&self) {
        /* Use BST to print the tree */
        let depth = self.depth();
        let mut queue = VecDeque::new();
        let mut index = VecDeque::new();
        queue.push_back(self);
        index.push_back(1);
        let mut last_index;
        let mut count = 0;
        let mut temp = 1;

        while count < depth {
            last_index = temp - 1;
            let mut size = queue.len();
            for _ in 0..temp {
                if size > 0 {
                    let mut i = index.front().unwrap();
                    // println!("I: [{}, {}]", last_index, i);
                    
                    last_index += 1;
                    if i > &(last_index) {
                        print!("nil ");
                        continue;
                    }

                    let i = index.pop_front().unwrap();
                    
                    let node = queue.pop_front().unwrap();
                    

                    print!("{} ", node.val);
                    if node.left.is_some() {
                        queue.push_back(node.left.as_ref().unwrap());
                        index.push_back(i * 2);
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.as_ref().unwrap());
                        index.push_back(i * 2 + 1);
                    }
                    size -= 1;
                } else {
                    print!("nil ")
                }
            }
            println!();

            temp *= 2;
            count += 1;
        }
    }
}

fn main() {
    let mut bst = BinarySearch::new(20);
    bst.insert(10);
    bst.insert(30);
    bst.insert(5);
    bst.insert(15);
    bst.insert(17);
    bst.insert(13);
    bst.insert(14);
    bst.insert(25);
    bst.insert(35);

    println!("depth: {}", bst.depth());
    println!("left: {}", bst.left.as_ref().unwrap().val);
    println!("right: {}", bst.right.as_ref().unwrap().val);
    println!(
        "search: {}",
        match bst.search(10) {
            Some(node) => "Find ".to_owned() + &node.val.to_string(),
            None => "Can not find".to_string(),
        }
    );
    println!(
        "search: {}",
        match bst.search(9999) {
            Some(node) => node.val.to_string(),
            None => "Can not find".to_string(),
        }
    );

    bst.print_tree();
    // bst.delete(35);
    // bst.print_tree();
    bst.delete(10);
    bst.print_tree();

    println!("End main");
}

#[cfg(test)]
mod tests {
    fn test_insert() {
        // let mut bst = BinarySearch::new(20);
        // bst.insert(10);
        // bst.insert(30);
        // bst.insert(5);
        // bst.insert(15);
        // bst.insert(25);
        // bst.insert(35);
        // bst.insert(50);
    }
}
