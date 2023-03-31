use std::io::Result;
fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();
    prost_build.btree_map(&["."]);
    prost_build.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    prost_build.compile_protos(&["src/pg_logicaldec.proto"], &["src/"])?;
    Ok(())
}
