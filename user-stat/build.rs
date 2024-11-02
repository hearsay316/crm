use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("./src/pb")?;
    let builder = tonic_build::configure();
    builder
        .out_dir("./src/pb")
        .with_serde(
            &["User"],
            true,
            true,
            Some(&[r#"
        #[serde(rename_all = "camelCase")]
        "#]),
        )
        .with_sqlx_from_row(&["User"], None)
        .with_derive_builder(
            &[
                "User",
                "QueryRequest",
                "RawQueryResult",
                "TimeQuery",
                "IdQuery",
            ],
            None,
        )
        .with_field_attributes(
            &["User.email", "User.name", "RawQueryRequest,query"],
            &[r#"
        #[builder(setter(into))]
        "#],
        )
        .with_field_attributes(
            &["TimeQuery.before", "TimeQuery,after"],
            &[r#"
        #[builder(setter(into,strip_option))]
        "#],
        )
        .compile_protos(
            &[
                "../protos/user-stats/message.proto",
                "../protos/user-stats/rpc.proto",
            ],
            &["../protos/user-stats"],
        )?;

    Ok(())
}
