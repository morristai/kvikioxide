fn main() -> miette::Result<()> {
    // let path = std::path::PathBuf::from("src");
    let kvikio_path = std::path::PathBuf::from("../kvikio/cpp/include");
    let bs_thread_pool_path = std::path::PathBuf::from("../kvikio/cpp/cmake-build-debug/_deps/bs_thread_pool-src/include");
    let nvtx3_path = std::path::PathBuf::from("../kvikio/cpp/cmake-build-debug/_deps/nvtx3-src/c/include");

    let mut builder = autocxx_build::Builder::new(
        "src/lib.rs",
        vec![&kvikio_path, &bs_thread_pool_path, &nvtx3_path]
    )
        .extra_clang_args(&["-std=c++17"])
        .build()
        .unwrap();

    builder.flag("-std=c++17")
        .cuda(true)
        .compile("kvikioxide");

    // TODO: can be folder?
    println!("cargo:rerun-if-changed=../kvikio/cpp/include/kvikio/file_handle.hpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
