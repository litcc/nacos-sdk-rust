
fn main() {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .build_transport(true)
        .out_dir("src")
        .compile_protos(&["./proto/nacos_grpc_service.proto"], &["./proto"])
        .unwrap();
}
