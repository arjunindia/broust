use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// Define the node structure
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    children: Vec<Rc<RefCell<TreeNode<T>>>>,
    parent: Option<Weak<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    // Create a new node
    fn new(value: T, parent: Option<Weak<RefCell<TreeNode<T>>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            children: Vec::new(),
            parent,
        }))
    }

    // Add a child to the node
    fn add_child(&mut self, child: Rc<RefCell<Self>>) {
        // Set the parent reference
        let weak_self = Rc::downgrade(&Rc::clone(&child));
        child.try_borrow_mut().unwrap().parent = Some(weak_self);

        // Push the child into the parent's children vector
        self.children.push(Rc::clone(&child));
    }
}
/*
 fn main() {
    // Create nodes
    let root = TreeNode::new("Root".to_string());
    let child1 = TreeNode::new("Child 1".to_string());
    let child2 = TreeNode::new("Child 2".to_string());

    // Add children to the root
    root.borrow_mut().add_child(Rc::clone(&child1));
    root.borrow_mut().add_child(Rc::clone(&child2));

    // Print the tree
    println!("{:#?}", root);
}
*/

/*
struct HTMLParser {
    body: String,
    unfinished: Vec<Element>,
}
impl HTMLParser {
    fn new(body: String) -> Self {
        Self {
            body,
            unfinished: Vec::new(),
        }
    }
    fn parse() {}
}
*/

enum Element {
    Tag(String),
    Text(String),
}
//struct DOM(Rc<RefCell<TreeNode<Element>>>);
/*
impl DOM {
    fn new() -> Self {
        let dom = TreeNode::new(Element::Tag("html".to_string()));
        DOM(dom)
    }
    fn insert_tag(&self, tag: String) {
        let node = TreeNode::new(Element::Tag(tag));
        &self.0.borrow_mut().add_child(Rc::clone(&node));
    }
    fn insert_textnode(&self, text: String) {
        let node = TreeNode::new(Element::Text(text));
        &self.0.borrow_mut().add_child(Rc::clone(&node));
    }
}
*/

struct HTMLParser {
    body: String,
    unfinished: Vec<Rc<RefCell<TreeNode<Element>>>>,
}
impl HTMLParser {
    pub fn new(body: String) -> Self {
        Self {
            body,
            unfinished: Vec::new(),
        }
    }
    fn add_text(&mut self, text: String) {
        let parent = self.unfinished.last_mut().unwrap();
        let node = TreeNode::new(Element::Text(text), Some(Rc::downgrade(&parent)));
        parent.try_borrow_mut().unwrap().add_child(node);
    }
    fn add_tag(&mut self, tag: String) {
        if tag.starts_with("/") {
            let node = self.unfinished.pop().unwrap();
            let mut parent = self
                .unfinished
                .last_mut()
                .unwrap()
                .try_borrow_mut()
                .unwrap();
            parent.add_child(node);
        } else {
            let parent = self.unfinished.last_mut();
            let node = TreeNode::new(
                Element::Tag(tag),
                match parent {
                    Some(parent) => Some(Rc::downgrade(&parent)),
                    None => None,
                },
            );
            self.unfinished.push(node);
        }
    }
    fn finish(&mut self) -> Rc<RefCell<TreeNode<Element>>> {
        while self.unfinished.len() > 1 {
            let node = self.unfinished.pop().unwrap();
            let mut parent = self
                .unfinished
                .last_mut()
                .unwrap()
                .try_borrow_mut()
                .unwrap();
            parent.add_child(node);
        }
        self.unfinished.pop().unwrap()
    }
    pub fn parse(&mut self) -> Rc<RefCell<TreeNode<Element>>> {
        let mut buffer = "".to_string();
        let mut in_tag = false;
        let chars = self.body.clone();
        let chars = chars.chars();
        for c in chars {
            if c == '<' {
                in_tag = true;
                if !buffer.is_empty() {
                    self.add_text(buffer);
                    /* output.push(Token::Text(Text {
                        text: buffer.clone(),
                    }));*/
                }
                buffer = "".to_string();
            } else if c == '>' {
                in_tag = false;
                self.add_tag(buffer);
                /*
                output.push(Token::Tag(Tag {
                    tag: buffer.clone(),
                }));*/
                buffer = "".to_string();
            } else {
                buffer = format!("{}{}", buffer, c);
            }
        }
        if !in_tag && !buffer.is_empty() {
            self.add_text(buffer);
            //  output.push(Token::Text(Text { text: buffer }))
        }
        self.finish()
    }
}
