fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("src/rpc/proto/broker.proto")?;
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .format(true)
        .out_dir("src/rpc")
        .compile(
            &["src/rpc/proto/broker.proto", "src/rpc/proto/client.proto"],
            &["src/rpc/proto"],
        )?;
    return Ok(());
}
