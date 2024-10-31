use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("./src/pb")?;
    let builder = tonic_build::configure();
    //  let mut  builder = Config::new();

    builder
        .out_dir("./src/pb")
        .compile_protos(&["../protos/crm/crm.proto"], &["../protos/crm"])?;
    Ok(())
}
