use provider::Output;
use std::result::Result;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::error::Error;

pub fn mode(name: &str) -> Result<Output, Box<Error>> {
    let res = try!(fs::metadata(name).map(|m| Output::U32(m.permissions().mode())));
    Ok(res)
}
