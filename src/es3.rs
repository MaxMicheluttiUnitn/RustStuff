#![warn(unused_imports)]
#![warn(dead_code)]
pub fn basicbox_sum(v: Vec<String>)->Vec<Box<usize>>{
    let mut res=vec![];
    let mut sum:usize=0;
    for x in v{
        sum+=x.len();
        res.push(Box::new(x.len()));
    }
    res.push(Box::new(sum));
    return res;
}