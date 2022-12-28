use crate::config::cfg::SAVE_DIR;
use std::{fs::File, io::{self, Write}};
use actix_web::web;


pub fn file_create(name:String)-> io::Result<File>{
    let filepath = std::path::Path::new(SAVE_DIR).join(name);
    File::create(&filepath)
}
pub fn file_add_buf(name:String, file:web::Bytes) -> Result<(), std::io::Error>{
    let filepath = std::path::Path::new(SAVE_DIR).join(name);
    let mut f = std::fs::OpenOptions::new().append(true).open(&filepath).unwrap();
    f.write_all(&file)
}