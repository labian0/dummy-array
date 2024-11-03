cd ../rust/ || exit; cargo clean; cargo build; mv target/debug/lib_dummy_arrays.so ../shared_libraries/; cd ../scripts/ || exit;
