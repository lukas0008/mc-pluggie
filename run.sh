cargo build --features init
cp ./target/debug/libmc_network.so ../pluggie/plugins/
cp ./target/debug/libmc_tick.so ../pluggie/plugins
cd ~/Code/rust/pluggie
cargo run -r
