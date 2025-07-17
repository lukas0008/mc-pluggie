cargo build --features init
cp ./target/debug/libmc_network.so ../pluggie/plugins/
cp ./target/debug/libmc_tick.so ../pluggie/plugins
cp ./target/debug/libmc_example.so ../pluggie/plugins
cp ./target/debug/libmc_loader.so ../pluggie/plugins
cp ./target/debug/libmc_status.so ../pluggie/plugins
cp ./target/debug/libmc_registry.so ../pluggie/plugins
cd ~/Code/rust/pluggie
cargo run -r
