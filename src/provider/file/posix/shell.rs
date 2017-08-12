use std::result::Result;
use std::error::Error;

use backend::Backend;
use provider::Output;

pub fn mode(name: &str, b: &Backend) -> Result<Output, Box<Error>> {
    let c = format!("stat -f%Lp {}", name);
    let res = try!(b.run_command(&c));
    let m = try!(u32::from_str_radix(&res, 8));
    Ok(Output::U32(m))
}
