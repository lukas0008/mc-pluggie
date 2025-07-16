RUSTFLAGS="-C prefer-dynamic" cargo build --features init
cp ./target/debug/libmc_network.dylib ../pluggie/plugins
cp ./target/debug/libmc_tick.dylib ../pluggie/plugins
cp ./target/debug/libmc_example.dylib ../pluggie/plugins
cp ./target/debug/libmc_loader.dylib ../pluggie/plugins
cp ./target/debug/libmc_status.dylib ../pluggie/plugins
cp ./target/debug/libmc_registry.dylib ../pluggie/plugins

cd ~/Code/rust/pluggie
cargo run -r
