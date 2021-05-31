fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "protos/tfplugin5.proto",
        ],
        &["protos"],
    )?;
    Ok(())
}
