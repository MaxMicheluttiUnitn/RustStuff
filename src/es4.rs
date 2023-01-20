#![warn(unused_imports)]
#![warn(dead_code)]
use std::fmt::{Display, Formatter};
use std::thread::current;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
#[derive(Debug)]
pub struct Content {
    s : String, b : bool, i : i32,
}
impl Content {
    pub fn new_with(s:String, b:bool, i:i32) -> Content {
        return Content{s,b,i};
    }
}


impl<T> List<T>{
    pub fn new() -> Self{
        List{head:None, len:0}
    }

    pub fn size(&self) -> i32{
        self.len.clone()
    }

    pub fn add(&mut self,e:T,p:i32) -> Result<(),String>{
        if p==0 {
            let temp=self.head.take();
            self.head=Some(Box::new(Node{
                elem: e,
                next: temp
            }));
            self.len+=1;
            return Ok(());
        }
        let mut current_link=&mut self.head;
        let mut id=p.clone()-1;
        while id>0{
            match current_link {
                None => {return Err("wrong position".to_string());},
                Some(node) => {current_link = &mut node.next;id-=1;}
            };
        }
        match current_link {
            Some(x)=>{
                let temp=x.next.take();
                x.next=Some(Box::new(Node{
                    elem: e,
                    next: temp
                }));
                self.len+=1;
                return Ok(());
            },
            None => Err("wrong position".to_string())
        }
    }

    pub fn prepend(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.len+=1;
        self.head = Some(new_node);
    }

    pub fn append(&mut self, elem:T){
        let mut current_node=&mut self.head;
        loop{
            match current_node {
                None => { *current_node=Some(Box::new(Node{next:None,elem:elem}));
                    self.len+=1;break;}
                Some(x) =>{
                    current_node=&mut x.next;
                }
            }
        }
    }

    pub fn get(&self,p: i32) -> Option<&T>{
        if p==0 {
            return match &self.head{
                None=>None,
                Some(x)=>Some(&x.elem)
            };
        }
        let mut id=p;
        let mut current_link=&self.head;
        while id>0{
            match current_link {
                None => {return None;},
                Some(node) => {current_link = &node.next;id-=1;}
            };
        }
        match current_link{
            None=>None,
            Some(x)=>Some(&x.elem)
        }
    }
}

impl<T:Display> List<T>{
    pub fn print(&self){
        let mut current_node=self.head.as_ref();
        println!("{}",self.len);
        while let Some(x)=current_node{
            print!("{} ",x.elem);
            current_node=x.next.as_ref();
        }
        println!()
    }
}
