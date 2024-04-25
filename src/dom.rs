use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::{Rc, Weak};

const SELF_CLOSING_TAGS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

pub struct TreeNode {
    pub value: Element,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    /** Create a new Tree Node */
    fn new(value: Element, parent: Option<Weak<RefCell<TreeNode>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            children: Vec::new(),
            parent,
        }))
    }

    /** Add a child to the node */
    fn add_child(&mut self, child: Rc<RefCell<Self>>) {
        // Set the parent reference
        let weak_self = Rc::downgrade(&Rc::clone(&child));
        child.try_borrow_mut().unwrap().parent = Some(weak_self);

        // Push the child into the parent's children vector
        self.children.push(Rc::clone(&child));
    }
}
pub struct Tag {
    pub tag: String,
    pub attributes: HashMap<String, String>,
}
pub enum Element {
    Tag(Tag),
    Text(String),
}
/** HTML Parser class. */
pub struct HTMLParser {
    body: String,
    unfinished: Vec<Rc<RefCell<TreeNode>>>,
}
impl HTMLParser {
    /** Creates an object for parsing */
    pub fn new(body: String) -> Self {
        Self {
            body,
            unfinished: Vec::new(),
        }
    }
    /** Add text node to the DOM */
    fn add_text(&mut self, text: String) {
        if text.trim().is_empty() {
            return;
        }
        let parent = self.unfinished.last_mut().unwrap();
        let node = TreeNode::new(Element::Text(text), Some(Rc::downgrade(&parent)));
        parent.try_borrow_mut().unwrap().add_child(node);
    }
    /** Add tag node to the DOM */
    fn add_tag(&mut self, text: String) {
        let (tag, attributes) = self.get_attributes(text);
        if tag.trim().is_empty() {
            return;
        }
        if tag.starts_with("!") {
            return;
        }
        if tag.starts_with("/") {
            if self.unfinished.len() == 1 {
                return;
            }
            let node = self.unfinished.pop().unwrap();
            let mut parent = self
                .unfinished
                .last_mut()
                .unwrap()
                .try_borrow_mut()
                .unwrap();
            parent.add_child(node);
        } else if SELF_CLOSING_TAGS.contains(&tag.as_str()) {
            let parent = self.unfinished.last_mut();
            let node = TreeNode::new(
                Element::Tag(Tag { tag, attributes }),
                match &parent {
                    Some(parent) => Some(Rc::downgrade(&parent)),
                    None => None,
                },
            );
            match parent {
                Some(parent) => parent.try_borrow_mut().unwrap().add_child(node),
                None => unreachable!(),
            };
        } else {
            let parent = self.unfinished.last_mut();
            let node = TreeNode::new(
                Element::Tag(Tag { tag, attributes }),
                match parent {
                    Some(parent) => Some(Rc::downgrade(&parent)),
                    None => None,
                },
            );
            self.unfinished.push(node);
        }
    }
    /** Get the attributes of a tag. */
    fn get_attributes(&self, text: String) -> (String, HashMap<String, String>) {
        let parts = text.split_whitespace().collect::<Vec<&str>>();
        let tag = parts.first().unwrap().to_lowercase();
        let mut attributes: HashMap<String, String> = HashMap::new();
        for attrpair in &parts[1..parts.len()] {
            if attrpair.contains("=") {
                let (key, mut value) = attrpair.split_once("=").unwrap();
                if value.len() > 2 && ["'", "\""].contains(&value.split_at(0).0) {
                    value = &value[1..value.len() - 1];
                }
                attributes.insert(key.to_lowercase(), value.to_string());
            } else {
                attributes.insert(attrpair.to_lowercase(), "".to_string());
            }
        }
        (tag, attributes)
    }
    /** finish parsing and return the root node */
    fn finish(&mut self) -> Rc<RefCell<TreeNode>> {
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
    /** Parse function. Parses the body of the object and returns the root node.*/
    pub fn parse(&mut self) -> Rc<RefCell<TreeNode>> {
        let mut buffer = "".to_string();
        let mut in_tag = false;
        let chars = self.body.clone();
        let chars = chars.chars();
        for c in chars {
            if c == '<' {
                in_tag = true;
                if !buffer.is_empty() {
                    self.add_text(buffer);
                }
                buffer = "".to_string();
            } else if c == '>' {
                in_tag = false;
                self.add_tag(buffer);
                buffer = "".to_string();
            } else {
                buffer = format!("{}{}", buffer, c);
            }
        }
        if !in_tag && !buffer.is_empty() {
            self.add_text(buffer);
        }
        self.finish()
    }
}
fn print_tree(node: &TreeNode, f: &mut fmt::Formatter<'_>, indent: i32) -> fmt::Result {
    for _ in 0..indent {
        write!(f, "\t").unwrap();
    }
    let _ = match &node.value {
        Element::Tag(t) => {
            write!(f, "<{}>\n", t.tag)
        }
        Element::Text(t) => {
            write!(f, "{}\n", t)
        }
    };
    for children in &node.children {
        let _ = children
            .try_borrow_mut()
            .inspect(|c| print_tree(c, f, indent + 1).unwrap());
    }
    Ok(())
}
impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_tree(self, f, 0)
    }
}
