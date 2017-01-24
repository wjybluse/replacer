use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
pub trait Cache{
    fn cache(&self)->HashMap<String,String>;
}

pub struct CacheProvider<'a>{
    file: &'a str,
}

impl<'a> CacheProvider<'a>{
    pub fn new(file: &'a str)->CacheProvider{
        CacheProvider{
            file: file,
        }
    }
}

impl<'a> Cache for  CacheProvider<'a> {
    fn cache(&self)->HashMap<String,String>{
        if self.file ==""{
            HashMap::new()
        }else{
            let mut f = File::open(self.file).unwrap();
            let mut buf_str = String::new();
            f.read_to_string(&mut buf_str).unwrap();
            let map :HashMap<String,String> = buf_str.replace("\r\n","\n").split("\n")
                    .filter(|item| !item.trim().is_empty() && !item.contains("#"))
                    .map(|item| pair(item) ).collect();
            map
        }
    }
}

fn pair(item: &str)->(String,String){
    let mut pair = item.split("=");
    (format!("{l}{key}{r}", l="${",key=pair.next().unwrap(),r="}"),pair.next().unwrap().to_string())
}
