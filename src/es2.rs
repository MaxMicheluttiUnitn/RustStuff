#![warn(dead_code)]
#![warn(unused_imports)]
pub struct Wrapper{
    v: Vec<i32>
}

pub struct Iter<'a>{
    v: &'a Vec<i32>,
    next: usize
}

impl<'a> Iterator for Iter<'a>{
    type Item = &'a i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next<self.v.len(){
            self.next+=2;
            self.v.get(self.next-2)
        }else{
            None
        }
    }
}

impl Wrapper{
    pub fn iter(&self)->Iter<'_>{
        Iter{v:&self.v,next:0}
    }
}