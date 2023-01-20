#![warn(unused_imports)]
#![warn(dead_code)]
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;
struct Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}
impl<T:Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
    }
}

#[derive(Debug)]
pub(crate) struct Graph<T> {
    nodes: Vec<NodeRef<T>>,
}
pub trait SameBool{
    fn samebool(&self, other:&Self)->bool;
}
#[derive(Debug)]
pub struct Content{
    pub i:i32,
    pub b:bool
}
impl Content {
    pub fn new_with(i: i32, b: bool) -> Content {
        Content { i, b }
    }
}

impl SameBool for Content{
    fn samebool(&self, other: &Self) -> bool {
        self.b==other.b
    }
}

impl PartialEq<Self> for Content {
    fn eq(&self, other: &Self) -> bool {
        self.i==other.i
    }
}

impl Eq for Content{
}

impl PartialOrd for Content{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.i.partial_cmp(&other.i)
    }

    fn lt(&self, other: &Self) -> bool {
        self.i<other.i
    }

    fn le(&self, other: &Self) -> bool {
        self.i<=other.i
    }

    fn gt(&self, other: &Self) -> bool {
        self.i>other.i
    }

    fn ge(&self, other: &Self) -> bool {
        self.i>=other.i
    }
}

impl<T> Graph<T>{
    pub fn new() -> Self{
        Graph{ nodes: vec![] }
    }
}

impl<T:PartialOrd+SameBool> Graph<T>{
    pub fn add_node(&mut self, elem:T){
        let mut node=Rc::new(RefCell::new(Node{ inner_value: elem, adjacent: vec![] }));
        self.nodes.push(node.clone());
        for x in self.nodes.iter(){
            let mut a=x.clone();
            let mut b=node.clone();
            if a.borrow().inner_value<b.borrow().inner_value{
                b.borrow_mut().adjacent.push(x.clone());
            }
            if a.borrow().inner_value.samebool(&(b.borrow().inner_value)){
                a.borrow_mut().adjacent.push(x.clone());
            }
        }
    }
}