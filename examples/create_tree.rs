use trees::*;

fn main() {
    let test = add_level(1, 4);
    println!("Hello, world! {:?}", test);
}

fn add_level(current_level : i32, max_level : i32) -> BTree0 {
    BTree0 {
        left: (
            if current_level < max_level{
                Some(Box::new(add_level(current_level + 1, max_level)))
            } else {
               None
            }
        ),
        right: (
            if current_level < max_level{
                Some(Box::new(add_level(current_level + 1, max_level)))
            } else {
               None
            }
        ),
        content: current_level
    }
}
