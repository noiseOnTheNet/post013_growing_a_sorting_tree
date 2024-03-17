use trees::*;

fn main() {
    let mut test = BTree1{ left: None, right: None, content: 5 };
    add_element(& mut test, 4);
    add_element(& mut test, 3);
    add_element(& mut test, 5);
    add_element(& mut test, 6);
    println!("Hello, world! {:?}", test);
}

fn add_element(node: & mut BTree1, value: i32){
    if node.content == value{
        // if the value is already in the tree do nothing
        return;
    } else if node.content < value {
        // check the left side for smaller values
        match node.left {
            None => {
                node.left = Some(Box::new(
                    BTree1 { left: None, right :None, content: value}
                ));
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
                node.left = Some(Box::new(
                    BTree1 { left: None, right :None, content: value}
                ));
            }
            Some(ref mut subnode) => {
                add_element(subnode, value);
            }
        }
    }
}
