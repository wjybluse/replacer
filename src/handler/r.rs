use std::path::Path;
use std::iter::Map;
use walkdir::{WalkDir,DirEntry};
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::{OpenOptions,File};
use mioco;
use std::io;
use regex::Regex;
pub trait Handler{
    //replace config item in each file
    fn replace(&self,dest:&str);
    //replace directory
}

#[derive(Clone)]
pub struct Replacer{
    cache: HashMap<String,String>,
    pattern: Regex,
}
impl Replacer{
    pub fn new(cache: HashMap<String,String>)->Replacer{
        Replacer{
            cache: cache,
            pattern: Regex::new(r"\$\{.+?\}").unwrap(),
        }
    }

    fn replace_dir(&self,dir: &str){
        let mut handlers = vec![];
        for entry in WalkDir::new(dir){
            let self_clone = self.clone();
            let h = mioco::spawn(move||->io::Result<()>{
                try!(self_clone.do_replace(&entry.unwrap(), |content| String::new()));
                Ok(())
            });
            handlers.push(h);
        }
        for h in handlers{
            match h.join(){
                Ok(_)=>self.do_something(),
                Err(err)=>println!("handle error message {:?}", err),
            }
        }
    }

    fn do_something(&self){
        //todo
    }
    fn replace_file(&self,file: &str){
        if let Some(r) = WalkDir::new(file).into_iter().next(){
            if let Ok(d) =  r{
                self.do_replace(&d, |content| String::new());
            }
        }
    }

    fn find_and_replace(&self,item: &str)->String{
        if item.starts_with("#")|| item.trim().is_empty() || !self.pattern.is_match(item){
            item.to_string()
        }else{
            let mut new_item :String = item.to_string();
            for cap in self.pattern.captures_iter(&item){
                let index = cap.iter()
                               .enumerate()
                               .find(|t|  t.1.is_some() )
                               .map(|t| t.0)
                               .unwrap_or(0);
                let key = cap.get(index)
                             .unwrap()
                             .as_str();
                if !self.cache.contains_key(key){
                    println!("the key is not exist {}",key );
                    continue;
                }
                new_item = new_item.replace(key,&self.cache[key]);
            }
            new_item
        }
    }

    fn do_replace<F>(&self,entry: &DirEntry,hook: F)->io::Result<()> where F: Fn(&str)->String{
        if entry.path().is_file(){
            let mut f = try!(OpenOptions::new()
                                    .read(true)
                                    .open(entry.path()));
            let mut buffer_str = String::new();
            try!(f.read_to_string(&mut buffer_str));
            if !buffer_str.contains("${"){
                Ok(())
            }else{
                //use hook?
                let new_buffer :String = buffer_str.replace("\r\n","\n")
                                                    .split("\n")
                                                    .map(|item| self.find_and_replace(item)+"\n")
                                                    .collect();
                let mut f1 = try!(File::create(entry.path()));
                try!(f1.write_all(new_buffer.as_bytes()));
                try!(f1.sync_all());
                Ok(())
            }
        }else{
            Ok(())
        }
    }
}

impl Handler for Replacer{
    fn replace(&self,dest:&str){
        let path = Path::new(dest);
        if path.is_dir(){
            if let Some(p) = path.to_str(){
                self.replace_dir(p);
            }
        }else {
            self.replace_file(dest);
        }
    }
}
