use std::fs::File;
use std::io::{prelude::*, Error};

pub fn load_from_file(path: String) -> Result<String,Error>{
    let mut f = File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}
