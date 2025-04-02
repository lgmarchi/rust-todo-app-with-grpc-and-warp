// This build script compiles the Protocol Buffers file located at "proto/todo.proto"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/todo.proto")?;
    Ok(())
}
