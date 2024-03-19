#[derive(Debug)]
pub struct BTree0 {
    pub left : Option<Box<BTree0>>,
    pub right : Option<Box<BTree0>>,
    pub content : i32,
}

#[derive(Debug)]
pub struct BTree1 {
    pub left : Option<Box<BTree1>>,
    pub right : Option<Box<BTree1>>,
    pub content : i32,
}


#[derive(Debug)]
pub struct BTree2<T> {
    pub left : Option<Box<BTree2<T>>>,
    pub right : Option<Box<BTree2<T>>>,
    pub content : T,
}
