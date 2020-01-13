
mod lfu {
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, Weak};
    use std::cell::RefCell;

    type Link = Option<Box<FrequencyNode>>;

    #[derive(Debug)]
    pub struct FrequencyNode {
        frequency: i32,
        prev: Option<Arc<RefCell<FrequencyNode>>>,
        next: Option<Arc<RefCell<FrequencyNode>>>,
        items: HashSet<String>,
    }

    #[derive(Debug)]
    pub struct Item {
        data: String,
        key: String,
        frequency_node: Arc<RefCell<FrequencyNode>>
    }

    impl Item {
        pub fn new (data: String, key: String, frequency_node: Arc<RefCell<FrequencyNode>>) -> Self {
            Item {data, key, frequency_node}
        }
    }

    #[derive(Debug)]
    pub struct LFU {
        frequency_list: Vec<Arc<RefCell<FrequencyNode>>>,
        frequency_head: Arc<RefCell<FrequencyNode>>,
        items: HashMap<String, Item>
    }

    impl FrequencyNode {
        pub fn new(frequency: i32, prev: Option<Arc<RefCell<FrequencyNode>>>, next: Option<Arc<RefCell<FrequencyNode>>>, items: HashSet<String>) -> Self {
            FrequencyNode {
                frequency,
                prev,
                next,
                items,
            }
        }
    }

    impl LFU {

        pub fn new() -> Self {
            let frequency_head = Arc::new(RefCell::new(FrequencyNode {
                frequency: 0,
                prev: None,
                next: None,
                items: HashSet::new()
            }));
            LFU {
                frequency_list: vec![frequency_head.clone()],
                frequency_head,
                items: HashMap::new()
            }
        }

        fn delete_item(&self, item:Item) {

        }

        pub fn set(&mut self, key: String, data:String) {
            if self.items.contains_key(&key) {
                let item = self.items.get(&key);
                item.unwrap().frequency_node.borrow_mut().items.remove(&key);
            }
            let new_frequency_node;
            if self.frequency_head.borrow().next.is_none(){
                new_frequency_node = Arc::new(RefCell::new(FrequencyNode {
                    frequency: 1,
                    prev: Some(self.frequency_head.clone()),
                    next: None,
                    items: HashSet::new()
                }));
            } else if self.frequency_head.borrow().next.as_ref().unwrap().borrow().frequency != 1 {
                new_frequency_node = Arc::new(RefCell::new(FrequencyNode {
                    frequency: 1,
                    prev: Some(self.frequency_head.clone()),
                    next: Some(self.frequency_head.borrow().next.as_ref().unwrap().clone()),
                    items: HashSet::new()
                }));
            } else {
                new_frequency_node = self.frequency_head.borrow().next.as_ref().unwrap().clone();
            }
            new_frequency_node.borrow_mut().items.insert(String::from(&key));
            self.items.insert(String::from(&key), Item::new(data, String::from(&key), new_frequency_node.clone()));
            self.frequency_list.push(new_frequency_node.clone());
            let node_a = self.frequency_head.clone();

            let node_b = Arc::new(RefCell::new(FrequencyNode {
                frequency: 1,
                prev: None,
                next: None,
                items: HashSet::new()
            }));
            node_a.borrow_mut().next = Some(node_b.clone());
            node_a.borrow_mut().next = Some(node_b.clone());
            node_a.borrow_mut().next = Some(node_b.clone());
        }

        pub fn get(&self, key: String) -> Option<String> {
            self.items.get(&key).map(|item|{
                String::from(&item.data)
            })
        }

        pub fn print_stats(&self) {
            println!("frequency list length is: {:?}", self.frequency_list.len());
            println!("items list length is: {:?}", self.items.len());
            println!("fhead is: {:?}", self.frequency_head);
            println!("first fnode is {:?}", self.frequency_list[0]);
            println!("second fnode is {:?}", self.frequency_list[1]);
            println!("third fnode is {:?}", self.frequency_list[2]);

            let node_a = Arc::new(RefCell::new(FrequencyNode {
                frequency: 1,
                prev: None,
                next: None,
                items: HashSet::new()
            }));

            let node_b = Arc::new(RefCell::new(FrequencyNode {
                frequency: 2,
                prev: None,
                next: None,
                items: HashSet::new()
            }));
            node_a.borrow_mut().next = Some(node_b.clone());
            node_b.borrow_mut().prev = Some(node_a.clone());
        }

    }

}

#[cfg(test)]
mod tests {
    use super::lfu::LFU;
    use super::lfu::FrequencyNode;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn basic () {
        let mut lfu = LFU::new();
        lfu.set("a".to_string(), "b".to_string());
        lfu.set("c".to_string(), "b".to_string());
        assert_eq!(lfu.get("a".to_string()), Some("b".to_string()));
        assert_eq!(lfu.get("a".to_string()), Some("b".to_string()));
        assert_eq!(lfu.get("a".to_string()), Some("b".to_string()));
        assert_eq!(lfu.get("a".to_string()), Some("b".to_string()));
        assert_eq!(lfu.get("c".to_string()), Some("b".to_string()));
        lfu.print_stats();
    }


}
