// This build script compiles the Protocol Buffers file located at "proto/todo.proto"
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/todo.proto")?;\
    println!(
        "cargo:warning=OUT_DIR is: {}",
        std::env::var("OUT_DIR").unwrap()
    );

    tonic_build::configure()
        .build_server(true)
        .compile(&["../proto/todo.proto"], &["../proto"])?;
    Ok(())
}
