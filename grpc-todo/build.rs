// This build script compiles the Protocol Buffers file located at "proto/todo.proto"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(&["../proto/todo.proto"], &["../proto"])?;
    Ok(())
}
