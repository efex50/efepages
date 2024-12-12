
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

        let (start_sector,mut ptr,end_sector) = {
            
            let arena_sector =start / self.pagesize;

            let end_sector = if start+write.len() == 0{
                0
            }
            else {
                // 1 eksiltiyoruz ki 1023 olsun
                (start + write.len() -1) / self.pagesize
            };

            let ptr = if start == 0 {
                0
            }
            else{
                start % self.pagesize
            };
            (arena_sector,ptr,end_sector)
        };
        let s_p:Vec<WritePages> = {
            let mut v: Vec<WritePages> = Vec::new();
            // tek page içindeyse
            if end_sector == start_sector {
                if self.page.get(&start_sector).is_none(){
                    v.push(WritePages{
                        contents:write,
                        sector:end_sector,
                        start:ptr
                    });
                }
            }


            // birden fazla page'de kalıyosa
            else {
                let mut s1 = self.pagesize - ptr;

                let end_size = (write.len() - ptr) % self.pagesize;
                
                let mut start = 0;
                let mut current_sector = start_sector;
                // daha düzgün bir apiyle yazılabilir 
                // mesela write[başla..son]
                // WritePages struct'ının içine end gömülebilir 
                // TODO!
                //for x in write{
                    //  if s1 == 0{
                    //      v.push(WritePages{
                    //          contents:buf.clone(),
                    //          sector:current_sector,
                    //          start:ptr
                    //      });
                    //      current_sector += 1;
                    //      buf.clear();
                    //      buf.push(x);
                    //      s1 = self.pagesize -1;
                    //      ptr = 0;
                    //  
                    //  }
                    //  else {
                    //      buf.push(x);
                    //      s1 -= 1;
                    //      
                    //  }
                //}

                
                loop {
                    let c = write[start..s1].to_vec();
                    v.push(WritePages { sector: current_sector, start: ptr, contents: c });
                    start = s1;
                    ptr = 0;
                    current_sector += 1;
                    if current_sector == end_sector{
                        s1 = start + end_size
                    }else if current_sector > end_sector {
                        break;
                    }
                    else {
                        s1 += self.pagesize;
                    }
                }

            }


            v

        };



        for x in s_p {
            let mut ctr: usize = x.start;
            if let Some(a) = self.page.get_mut(&x.sector) {
                for xx in x.contents {
                    a[ctr] = xx;
                    ctr += 1;
                }
                ctr = 0;
            }else{
                self.create_new_page(x.sector);
                let mut v = self.page.get_mut(&x.sector).unwrap();

                let mut v = vec![0;self.pagesize];
                for xx in x.contents {
                    v[ctr] = xx;
                    ctr += 1;
                }
                ctr = 0;
                self.page.insert(x.sector, v);
            }
        }
        
        
    }
    
    














    pub fn read(&self,start:usize,end:usize) -> Vec<u8>{
        let (start_sector,mut ptr,end_sector) = {
            
            let arena_sector =start / self.pagesize;

            let end_sector = if start+end == 0{
                0
            }
            else {
                // 1 eksiltiyoruz ki 1023 olsun
                (start + end -1) / self.pagesize
            };

            let ptr = if start == 0 {
                0
            }
            else{
                start % self.pagesize
            };
            (arena_sector,ptr,end_sector)
        };
        let mut readpages: Vec<ReadPages> = Vec::new();


        if end_sector == start_sector{

            readpages.push(ReadPages{
                end:ptr + end,
                sector:start_sector,
                start:ptr
            });

        }else {

        }

        dbg!(&readpages);

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
}