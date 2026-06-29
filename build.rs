fn main() {
    println!("cargo:rerun-if-changed=assets/douyin.proto");
    let file_descriptors = protox::compile(["assets/douyin.proto"], ["assets"])
        .expect("compile douyin.proto descriptors");
    prost_build::compile_fds(file_descriptors).expect("compile douyin.proto");
}
