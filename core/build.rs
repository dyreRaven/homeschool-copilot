fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["../proto/health.proto"],
            &["../proto"],
        )?;
    println!("cargo:rerun-if-changed=../proto/health.proto");
    Ok(())
}
