use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};

const VERBOSE: bool = false;

pub struct Node {
    key: u64,
    value_size: usize,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

pub struct LRUCache {
    capacity: usize,            // Maximum allowed size of cache
    current_size: usize,        // Current size of all objects in the cache
    map: HashMap<u64, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    misses: usize,
    evicted_bandwidth: usize,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            current_size: 0,
            map: HashMap::new(),
            head: None,
            tail: None,
            misses: 0,
            evicted_bandwidth: 0,
        }
    }

    pub fn get(&mut self, key: u64) -> Option<usize> {
        let maybe_node = self.map.get(&key).cloned();
        maybe_node.map(|node| {
            self.move_to_front(&node);
            node.borrow().value_size
        }).or_else(|| {
            self.misses += 1;
            None
        })
    }

    pub fn put(&mut self, key: u64, value_size: usize) {
        let maybe_node = self.map.get(&key).cloned();
        if let Some(_node) = maybe_node {
            // Update current size if the value size changes
            // let mut node_ref = node.borrow_mut();
            // self.current_size += value_size - node_ref.value_size; 
            // node_ref.value_size = value_size;
            // self.move_to_front(&node);
            panic!("Key already exists in the cache: {}", key);
        } else {
            // check if it is cache-able
            if self.capacity < value_size {
                return;
            }
            self.evict(value_size);
            let new_node = Rc::new(RefCell::new(Node { key, value_size, prev: None, next: None }));
            self.map.insert(key, Rc::clone(&new_node));
            self.insert_front(&new_node);
            self.current_size += value_size;  // Increase current size
            if VERBOSE {
                println!("Inserted key: {}, size: {} || {} -> {} || map len: {} || head: {}, tail: {}",
                    key, value_size, self.current_size - value_size, self.current_size, self.map.len(),
                    self.head.as_ref().map(|x| x.borrow().key).unwrap_or(0),
                    self.tail.as_ref().map(|x| x.borrow().key).unwrap_or(0));
            }
        }
    }

    fn insert_front(&mut self, node: &Rc<RefCell<Node>>) {
        // assert if it's a new node
        assert!(node.borrow().prev.is_none() && node.borrow().next.is_none());
        node.borrow_mut().next = self.head.clone();
        node.borrow_mut().prev = None;
        
        if let Some(ref head) = self.head {
            head.borrow_mut().prev = Some(Rc::clone(node));
        }
        
        self.head = Some(Rc::clone(node));
        
        if self.tail.is_none() {
            self.tail = Some(Rc::clone(node));
        }
    }

    fn move_to_front(&mut self, node: &Rc<RefCell<Node>>) {
        // check if it is already at the head
        if let Some(ref head) = self.head {
            if Rc::ptr_eq(head, node) {
                return;
            }
        }
        let next_link = node.borrow_mut().next.take();
        let prev_link = node.borrow_mut().prev.take();
        
        if let Some(ref next) = next_link {
            next.borrow_mut().prev = prev_link.clone();
            if VERBOSE {
                println!("Next key: {}, Next->prev: {}", next.borrow().key, next.borrow().prev.as_ref().map(|x| x.borrow().key).unwrap_or(0));
            }
        } else {
            assert!(Rc::ptr_eq(self.tail.as_ref().unwrap(), node));
        }
        
        if let Some(ref prev) = prev_link {
            prev.borrow_mut().next = next_link.clone();
            if VERBOSE {
                println!("Prev key: {}, Prev->next: {}", prev.borrow().key, prev.borrow().next.as_ref().map(|x| x.borrow().key).unwrap_or(0));
            }
            if next_link.is_none() {
                self.tail = Some(Rc::clone(prev));
            }
        } else {
            panic!("It must not be the head");
        }
        
        node.borrow_mut().next = self.head.clone();
        node.borrow_mut().prev = None;
        if VERBOSE {
            println!("Node key: {}, Node->next: {}", node.borrow().key, node.borrow().next.as_ref().map(|x| x.borrow().key).unwrap_or(0));
        }
        
        if let Some(ref head) = self.head {
            head.borrow_mut().prev = Some(Rc::clone(node));
            if VERBOSE {
                println!("Head key: {}, Head->prev: {}", head.borrow().key, head.borrow().prev.as_ref().map(|x| x.borrow().key).unwrap_or(0));
            }
        }
        
        self.head = Some(Rc::clone(node));
    }

    fn evict(&mut self, free_space: usize) {
        let mut evt_count = 0;
        // let cur_size_before_eviction = self.current_size;
        while self.current_size + free_space > self.capacity {
            self.evict_one();
            evt_count += 1;
            if evt_count > 1000 {
                if let Some(ref tail) = self.tail {
                    if VERBOSE {
                        println!("Tail key: {}, Tail->prev: {}", tail.borrow().key, tail.borrow().prev.as_ref().map(|x| x.borrow().key).unwrap_or(0));
                        if let Some(prev) = tail.borrow().prev.as_ref() {
                            println!("Tail->prev->next: {:?}", prev.borrow().key);
                        }
                    }
                }
                // panic!("Eviction loop detected {}, curr_size {} -> {}, free_space: {}, map_len {}",
                //     evt_count, cur_size_before_eviction, self.current_size, free_space, self.map.len());
            }
        }
    }
    fn evict_one(&mut self) {
        if let Some(ref tail) = self.tail {
            let tail_key = tail.borrow().key;
            let tail_prev = tail.borrow_mut().prev.take();
            
            if let Some(prev) = tail_prev {
                if VERBOSE {
                    println!("Tail key: {}, Tail->prev: {}", tail_key, prev.borrow().key);
                }
                prev.borrow_mut().next = None;
                self.tail = Some(prev);
            } else {
                assert!(self.head.is_some() && Rc::ptr_eq(self.head.as_ref().unwrap(), tail));
                self.head = None;
                self.tail = None;
            }
            
            if let Some(node) = self.map.remove(&tail_key) {
                let node_ref = node.borrow();
                self.current_size -= node_ref.value_size;  // Decrease current size
                self.evicted_bandwidth += node_ref.value_size;  // Update evicted bandwidth
                if node_ref.value_size <= 0 {
                    panic!("Node size is less than or equal to 0 :: key: {}, size: {}", tail_key, node_ref.value_size);
                } else {
                    if VERBOSE {
                        println!("Evicted key: {}, size: {} || {} -> {} || map len: {} || head: {}, tail: {}",
                            tail_key, node_ref.value_size, self.current_size + node_ref.value_size,
                            self.current_size, self.map.len(),
                            self.head.as_ref().map(|x| x.borrow().key).unwrap_or(0),
                            self.tail.as_ref().map(|x| x.borrow().key).unwrap_or(0));
                    }
                }
            } else {
                panic!("Node not found in map :: key: {}", tail_key);
            }
        } else {
            panic!("Tail is None, but current size: {}", self.current_size);
        }
    }
}
