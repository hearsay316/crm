use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("./src/pb")?;
    let builder = tonic_build::configure();
    builder
        .out_dir("./src/pb")
        .with_serde(
            &["Content","ContentType","Publisher","Timestamp"],
            true,
            true,
            Some(&[r#"
        #[serde(rename_all = "camelCase")]
        "#]),
        )
        .compile_protos(
            &[
                "../protos/metadata/messages.proto",
                "../protos/metadata/rpc.proto",
            ],
            &["../protos/metadata"],
        )?;

    Ok(())
}
