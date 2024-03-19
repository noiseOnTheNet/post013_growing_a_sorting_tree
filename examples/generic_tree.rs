use trees::*;
use std::fmt::Display;

fn main() {
    let mut test = make_leaf("orange");
    for i in ["banana","apple","orange","mango"]{
        add_element(& mut test, i);
    }
    // this will print 3 4 5 6
    deep_traversal_print(& test);
}

fn add_element<T: Ord>(node: & mut BTree2<T>, value: T){
    if node.content == value{
        // if the value is already in the tree do nothing
        return;
    } else if node.content > value {
        // check the left side for smaller values
        match node.left {
            None => {
                node.left = Some(Box::new(make_leaf(value)));
            }
            // a tricky part: we need to tell the compiler to return a
            // mutable reference from this pattern match otherwise it
            // may try to move the ownership of the data (which we don't want)
            Some(ref mut subnode) => {
                add_element(subnode, value);
            }
        }
    } else {
        match node.right {
            None => {
                node.right = Some(Box::new(make_leaf(value)));
            }
            Some(ref mut subnode) => {
                add_element(subnode, value);
            }
        }
    }
}

fn make_leaf<T>(value: T) -> BTree2<T>{
    BTree2 { left: None, right :None, content: value}
}

fn deep_traversal_print<T: Display>(node: & BTree2<T>){
    if let Some(ref subnode) = node.left {
        deep_traversal_print(subnode);
    }
    print!("{} ",node.content);
    if let Some(ref subnode) = node.right {
        deep_traversal_print(subnode);
    }
}
