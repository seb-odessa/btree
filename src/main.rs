use std::fmt::{Display, Formatter};
use std::fmt;

type K = u32;
type V = String;

#[allow(dead_code)]
struct Payload {
    key  : K,
    data : V,
}
#[allow(dead_code)]
impl Payload {
    pub fn new(key : K, content : &str) -> Payload {
        Payload{ key : key, data : content.to_string() }
    }
}
impl Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

struct Node {
    value: Payload,
    left : MaybeNode,
    right: MaybeNode,
}
type MaybeNode = Option<Box<Node>>;
impl Node {
    pub fn new(item : Payload) -> MaybeNode {
        let node = Node { 
            value : Payload {key : item.key, data : item.data.clone()}, 
            left  : None, 
            right : None 
        };
        Some(Box::new(node))
    }
    
    pub fn insert(&mut self, value : Payload) {
        if value.key > self.value.key {
            self.insert_right(value)
        } else if value.key < self.value.key {
            self.insert_left(value)
        } else {
            self.value = value
        }
    }
    
    fn insert_right(&mut self, value : Payload) {
        //println!("{}.insert_right({})", self.value, value);
        match self.right {
            Some(ref mut branch) => branch.insert(value),
            None => self.right = Node::new(value)
        }
    }
    
    fn insert_left(&mut self, value : Payload) {
        //println!("{}.insert_left({})", self.value, value);
        match self.left {
            Some(ref mut branch) => branch.insert(value),
            None => self.left = Node::new(value)
        }
    }

    pub fn find<'a>(&'a mut self, key : K) -> Option<&'a mut Node> {
        //println!("({}).find({})", self.value, key);
        if key == self.value.key { return Some(self) }
        if key > self.value.key {
            if let Some(ref mut right) = self.right { return right.find(key) }
        } else {
            if let Some(ref mut left) = self.left { return left.find(key) }
        }
        return None
    }
    
    pub fn update(&mut self, value : V) {
        self.value.data = value;
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
    root : MaybeNode,
}
impl BTree {
    pub fn new() -> BTree {
        BTree{root : None}
    }
    
    pub fn insert(&mut self, value : Payload) {
        match self.root {
            Some(ref mut root) => root.insert(value),
            None => self.root = Node::new(value)
        };
    }

    pub fn find<'a>(&'a mut self, key : K) -> Option<&'a mut Node> {
        match self.root {
            Some(ref mut root) => root.find(key),
            None => None
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
    let mut tree = BTree::new();
    tree.insert(Payload::new(1, "A"));
    tree.insert(Payload::new(2, "B"));
    tree.insert(Payload::new(3, "C"));
    tree.insert(Payload::new(4, "D"));
    tree.insert(Payload::new(5, "E"));
    tree.insert(Payload::new(6, "F"));
    tree.insert(Payload::new(7, "G"));

    println!("{}", tree);
    
    for i in 1..10 {
        match tree.find(i) {
            Some(v) => println!("The {0} was found in the tree.", v), 
            None => println!("The item with key {0} was not found in the tree.", i), 
        }
    }

    match tree.find(4) {
        Some(mut node) => node.update("Z".to_string()),
        None => {}
    }

    println!("{}", tree);
    
    println!("{}", tree);
    
}







