use std::io::prelude::*;
use std::fs::File;
use num::{BigInt, Num};
use graphics::types::Color;

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern,
// mostly for external type to type From/TryFrom conversions
pub struct W<T>(pub T);

// Personal preference.
pub use std::format as f;
pub struct Tools{}
trait Imports {
    fn convert_hex_to_dec(hex_str: &str) -> String;
    fn convert_rgb(_str0:String)->[f32; 3];
    fn get_rgb(_str0:String)->Vec<f32>;
    fn Scolor(r:f32,g:f32,b:f32,a:f32) ->Color;
    fn verify()->bool;
    fn load_file(path:&str)->Vec<String>;
    fn new()->Tools;
    fn shexcolor(hex0:&str,a:f32) ->Color;
    fn string_vec(string:  String)->Vec<String>;
    fn string_to_str<'a>(_str:&'a String)->&str;
    
}
impl Imports for Tools {
    fn string_vec(string:  String)->Vec<String> {
         let result=vec![string];
         return result;
    }

    fn string_to_str<'a>(_str:&'a String)->&str {
        let read_line =&_str[0.._str.len()];
        return read_line;
    }

    fn new()->Tools {
        return Tools {}
    }

    fn load_file(path:&str)->Vec<String>{
        let _file=File::open("")
        .expect("not find file");
        let mut cntx=String::new();
        let txt=_file.read_to_string(&mut cntx)
        .expect("msg->load file failed");
        let false_=vec!["false_".to_owned()];
        return false_;
    }

    fn verify()->bool {
        todo!()
    }
     fn Scolor(r:f32,g:f32,b:f32,a:f32) ->Color  
{
    let COLOR: Color = [r,g,b,a];
    return COLOR;
}
  fn shexcolor(hex0:&str,a:f32) ->Color
{   let hex=hex0.to_string();
    let v=Self::get_rgb(hex);
    let color: Color = [v[0],v[1],v[2],a];
    return color;
}

fn convert_hex_to_dec(hex_str: &str) -> String {
    BigInt::from_str_radix(hex_str, 16).unwrap().to_string()
    }

fn convert_rgb(_str0:String)->[f32; 3] {
    let _str:String=_str0.replace("#","");
    let r:String=_str[0..2].to_string();
    let _r_int:String=Self::convert_hex_to_dec(&r);
    let g:String=_str[2..4].to_string();
    let _g_int:String=Self::convert_hex_to_dec(&g);
    let b:String=_str[4..6].to_string();
    let _b_int:String=Self::convert_hex_to_dec(&b);
    let list_int=[_r_int.parse::<f32>().unwrap(),
    _g_int.parse::<f32>().unwrap(),
    _b_int.parse::<f32>().unwrap()];
    return list_int;
    }

fn get_rgb(_str0:String)->Vec<f32> {
    let _str:String=_str0.replace("#","");
    let r:String=_str[0..2].to_string();
    let _r_int:String=Self::convert_hex_to_dec(&r);
    let g:String=_str[2..4].to_string();
    let _g_int:String=Self::convert_hex_to_dec(&g);
    let b:String=_str[4..6].to_string();
    let _b_int:String=Self::convert_hex_to_dec(&b);
    let rgb:Vec<f32>=vec![_r_int.parse::<f32>().unwrap(),
    _g_int.parse::<f32>().unwrap(),
    _b_int.parse::<f32>().unwrap()];
    return rgb;
    }
}
