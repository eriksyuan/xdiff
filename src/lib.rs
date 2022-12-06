mod config;
pub mod cli;
pub mod req;
pub use config::{DiffConfig,DiffProfile,RequestProfile,ResponseProfile};


#[derive(Debug,Clone)]
pub struct ExtraArgs{
  pub headers:Vec<(String,String)>,
  pub body:Vec<(String,String)>,
  pub query:Vec<(String,String)>,  
}