cd ./rust/ || exit; cargo clean;
cargo build;
mv target/debug/lib_dummyarray_rust.so ../shared_libraries/;
