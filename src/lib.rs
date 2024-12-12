pub mod page;
pub mod vars;










#[cfg(test)]
mod tests{
    use std::usize;

    use crate::{page::Page, vars::PAGE_SIZE};
    #[test]
    fn paget1(){
        let mut p = Page::default();
        p.page.insert(10, vec![12,32,32,3]);
        p.page.insert(15, vec![12,32,32,3]);
        p.page.insert(usize::MAX, vec![12,32,32,3]);
        println!("{:?}",p)
    }
    #[test]
    fn pt2(){
        println!("{}",1025 % PAGE_SIZE)
    }
    #[test]
    fn pt3(){
        let mut page = Page::default();
        page.write(24, vec![20;10000]);
        println!("{:?}",page);
        println!("{:?}",page.page.get(&0).unwrap().len());
    }
    #[test]
    fn pt4(){
        let a = vec![20;200];
        let b = a[0..20].to_vec();
        println!("{:?}",b[19]);
        println!("len {}",b.len())
        
    }

    #[test]
    fn pt5(){
        let mut p = Page::default();
        p.write(2481152 - 100, vec![10;100]);
        println!("{:?}",p);

    }

    #[test]
    fn pt6(){
        let p = Page::default();
        let r = p.read(1000, 24);
        println!("{:?}",r);
    }
    #[test]
    fn pt7(){
        let mut p = Page::default(); 
        p.write(250, vec![31;500]);
        let r = p.read(730, 24);
        println!("{:?}",r);
    }

    #[test]
    fn pt8(){
        let mut p = Page::default();
        let len = 2400;
        p.write(250, vec![31;50000]);
        let r = p.read(1000, len);
        println!("{:?}",r);
        assert_eq!(r.len() , len);

    }

    #[test]
    fn pt9(){
        let mut p = Page::default();
        p.write(0, vec![10;10]);

        p.write(5, vec![11;10]);
        println!("{:?}",p)
    }

    #[test]
    fn malloc(){
        let mut p = Page::default();
        p.write(0, vec![10;100]);
        dbg!("OK!");
        p.write(18446744073709550592 - 200, vec![255;200]);
        println!("{:?}",p);

    }
}