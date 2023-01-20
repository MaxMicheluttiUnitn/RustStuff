#![warn(unused_imports)]
#![warn(dead_code)]
use std::fmt::Debug;

pub trait Nextable{
    type Item:Debug;
    fn gimme_next(&self)->Option<Self::Item>;
}

impl Nextable for i32{
    type Item=i32;
    fn gimme_next(&self) -> Option<Self::Item> {
        return Some(self+1);
    }
}

impl Nextable for char{
    type Item=char;
    fn gimme_next(&self) -> Option<Self::Item> {
        let mut asu32=*self as u32;
        asu32+=1;
        let res=char::from_u32(asu32);
        return res;
    }
}

pub fn printnext(x: impl Debug+Nextable){
    let y=x.gimme_next();
    match y{
        Some(next)=>{
            println!("Next of {:?} is Some({:?})",x,next);
        },
        None=>{
            println!("Next of {:?} is None",x);
        }
    }
}