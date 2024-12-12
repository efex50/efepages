pub mod init;
pub mod funs;

use std::{collections::HashMap, hash::BuildHasherDefault};

use nohash_hasher::NoHashHasher;

pub type PageMap = HashMap::<usize, Vec<u8>, BuildHasherDefault<NoHashHasher<usize>>>;

#[derive(Debug)]
pub struct Page{
    pagesize:usize,
    pub page:PageMap,
    max_pages:Option<usize>
}


