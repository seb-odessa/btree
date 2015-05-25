use std::fmt::{Display, Formatter};
use std::fmt;

 #[allow(dead_code)]
struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    pub fn new(value : u32) -> Box<Node> {
        Box::new(Node{value : value, left : None, right : None})
    }
    
    pub fn insert(&mut self, value : u32) {
        if value > self.value {
            self.insert_right(value)
        }
        if value < self.value {
            self.insert_left(value)
        }
    }
    
    fn insert_right(&mut self, value : u32) {
        println!("{}.insert_right({})", self.value, value);
        match self.right {
            Some(ref mut branch) => branch.insert(value),
            None => self.right = Some(Node::new(value))
        }
    }
    
    fn insert_left(&mut self, value : u32) {
        println!("{}.insert_left({})", self.value, value);
        match self.left {
            Some(ref mut branch) => branch.insert(value),
            None => self.left = Some(Node::new(value))
        }
    }
    
    pub fn find(&self, value : u32) -> bool {
        println!("{}.find({})", self.value, value);
        if value == self.value { return true }
        if value > self.value {
            self.find_right(value)
        } else {
            self.find_left(value)
        }
    }
    
    fn find_left(&self, value : u32) -> bool {
        match self.left {
            Some(ref branch) => branch.find(value),
            None => false
        }
    }
    
    fn find_right(&self, value : u32) -> bool {
        match self.right {
            Some(ref branch) => branch.find(value),
            None => false
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lhv = match self.left {
            Some(ref branch) => format!("{}", branch),
            None => format!("")
        };
    
        let rhv = match self.right {
            Some(ref branch) => format!("{}", branch),
            None => format!("")
        };
        
        write!(f, "({}<{}>{})", lhv, self.value, rhv)
    }
}

#[allow(dead_code)]
struct BTree {
    root : Option<Box<Node>>,
}
impl BTree {
    pub fn new(value : u32) -> BTree {
        BTree{root : Some(Node::new(value))}
    }
    
    pub fn insert(&mut self, value : u32) {
        match self.root {
            Some(ref mut root) => root.insert(value),
            None => self.root = Some(Node::new(value))
        };
    }
    
    pub fn find(&self, value : u32) -> bool {
        match self.root {
            Some(ref root) => root.find(value),
            None => false
        }
    }
}
impl Display for BTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.root {
            Some(ref root) => write!(f, "{}", root),
            None => write!(f, "()")
        }
    }
}

fn main() {
    let mut tree = BTree::new(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(4);
    tree.insert(2);
    tree.insert(6);
    tree.insert(8);

    println!("{}", tree);
    for i in 1..10 {
        println!("The {0} was {1} found in the tree.", i, 
            if tree.find(i) {""} else {"not"}
        );
    }
}







