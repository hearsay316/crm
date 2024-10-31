use anyhow::Result;
use std::fs;
use prost_build::Config;

fn main() -> Result<()> {
    fs::create_dir_all("./src/pb")?;
    // let builder = tonic_build::configure();
    // builder
    //     .out_dir("./src/pb")
    //     .compile_protos(&["../protos/user-stats/*.proto"], &["../protos/user-stats"])?;

    Config::default().out_dir("./src/pb");
    Ok(())
}
