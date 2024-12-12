
use super::Page;


#[derive(Debug)]
struct WritePages{
    sector:usize,
    start:usize,
    contents:Vec<u8>
}

#[derive(Debug)]
struct ReadPages{
    sector:usize,
    start:usize,
    end:usize
}

impl Page{
    /// writes from start to end of the given vector
    /// 
    /// ```
    /// use efepages::page::Page;
    /// let mut p = Page::default();
    /// p.write(20,vec![255;500]);
    /// ```
    pub fn write(&mut self,start:usize,write:Vec<u8>){
        let v = self.get_writepages(start, write);
        for x in v {

            let mut ctr: usize = x.start;
            
            if let Some(a) = self.page.get_mut(&x.sector) {
                for xx in x.contents {
                    a[ctr] = xx;
                    ctr += 1;
                }
            }else{
                self.create_new_page(x.sector);
                
                let v = self.page.get_mut(&x.sector).unwrap();
                
                for xx in x.contents {
                    v[ctr] = xx;
                    ctr += 1;
                }
            }
        }
        
    }
    fn get_writepages(&self,start:usize, write:Vec<u8>) -> Vec<WritePages>{
        let (start_page,mut ptr,end_page) = self.get_vars(start, write.len());
        let mut v: Vec<WritePages> = Vec::new();

        
        if end_page == start_page{
            v.push(WritePages{
                contents:write,
                sector:end_page,
                start:ptr
            });
        }else {
            // head
            let tail = start + write.len();
            let mut current_page = start_page;
            let end_ptr = self.pagesize;
            let mut counter = 0;
            let mut counter_end = end_ptr - ptr;
            loop {
                v.push(WritePages{
                    contents:write[counter..counter_end].to_vec(),
                    sector:current_page,
                    start:ptr
                });
                current_page += 1;
                ptr = 0;

                // tail
                if current_page == end_page{
                    counter = counter_end;
                    counter_end = counter + (tail % self.pagesize);
                }
                //break
                else if current_page > end_page{
                    break;
                }
                // body
                else {
                    counter = counter_end;
                    counter_end += self.pagesize;
                }
            }
        }
        v
    }
    


    pub fn read(&self,start:usize,len:usize) -> Vec<u8>{
        let (start_sector,mut ptr,end_sector) = self.get_vars(start, len);
        let mut readpages: Vec<ReadPages> = Vec::new();
        if end_sector == start_sector{
            
            readpages.push(ReadPages{
                end:ptr + len,
                sector:start_sector,
                start:ptr
            });
            
        }else {
            let tail = start + len;
            let mut current_sector = start_sector;
            let mut end_ptr = self.pagesize;
            loop {
                readpages.push(ReadPages{
                    sector:current_sector,
                    start:ptr,
                    end: end_ptr,
                });
                ptr = 0;
                current_sector += 1;

                // last loop
                if current_sector == end_sector{
                    end_ptr = tail % self.pagesize;
                }
                // break the loop
                else if current_sector > end_sector {
                    break;
                }
                // body
                else {
                    end_ptr = self.pagesize;
                }

            }


        }
        let mut readed: Vec<u8> = Vec::new();
        for x in readpages {
            if let Some(page) = self.page.get(&x.sector){
                readed.append(&mut page[x.start..x.end].to_vec());
            }else { 
                readed.append(&mut vec![0;x.end - x.start]);
            }
        }


        readed
    }












    fn get_vars(&self,start:usize,len:usize) -> (usize,usize,usize){
        let arena_sector =start / self.pagesize;

        let end_sector = if start+len == 0{
            0
        }
        else {
            // 1 eksiltiyoruz ki 1023 olsun
            (start + len -1) / self.pagesize
        };

        let ptr = if start == 0 {
            0
        }
        else{
            start % self.pagesize
        };
        (arena_sector,ptr,end_sector)

    }
}