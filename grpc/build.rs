fn main() {
    tonic_build::compile_protos("./hello.proto").unwrap();
}
