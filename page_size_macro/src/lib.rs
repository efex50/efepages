extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn page_size(_i:TokenStream) -> TokenStream{
    // let a = env!("efearena_size");
    let a = std::env::var("efepage_size").unwrap_or("1024".to_string());
    let s = format!("pub const PAGE_SIZE:usize = {};",a);
    s.parse().unwrap()
}