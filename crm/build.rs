use std::env;
use anyhow::Result;

fn main() -> Result<()> {
    println!("运行的目录 {:?}", env::current_dir());
    prost_build::compile_protos(&["../protos/crm.proto"],&["../protos"])?;
    Ok(())
}