use std::{collections::HashMap, hash::BuildHasherDefault};

use crate::vars::PAGE_SIZE;

use super::{Page, PageMap};


impl Page {
    pub fn new_with_custom_pagesize(pagesize:usize) -> Self{
        let page:PageMap =
        HashMap::with_capacity_and_hasher(2, BuildHasherDefault::default());
        Self { page, pagesize ,..Default::default()}
    }
    pub fn set_max_pages(&mut self){
        todo!()
    }


    pub(crate) fn create_new_page(&mut self,page:usize){
        if let Some(max) = self.max_pages{
            match max < self.page.len() {
                true => self.add_page(page),
                false => panic!("Maximum pages reached"),
            }
        }else {
            self.add_page(page);
        }
    }
    
    fn add_page(&mut self,page:usize){
        let v = vec![0;self.pagesize];
        self.page.insert(page, v);
    }

}


impl Default for Page{
    fn default() -> Self {
        let page:PageMap =
        HashMap::with_capacity_and_hasher(2, BuildHasherDefault::default());
        Self { page, pagesize: PAGE_SIZE ,max_pages:None}
    }
}

