use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        todo!()
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        todo!()
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        todo!()
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    todo!()
}