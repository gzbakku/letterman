mod build;
mod execute;
mod parse;
mod resolve;
pub use build::Email;

#[derive(Debug,Clone)]
pub struct Action {
    pub tag:&'static str,
    pub cate:&'static str,
    pub io:&'static str,
    pub cmd:String
}