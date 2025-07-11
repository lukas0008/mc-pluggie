cargo build -r -p mc-network --features mc-network/init
cargo build -r -p mc-tick --features mc-tick/init
cargo build -r -p mc-example --features mc-example/init

cp ./target/release/libmc_network.so ../pluggie/plugins/
cp ./target/release/libmc_tick.so ../pluggie/plugins
cp ./target/release/libmc_example.so ../pluggie/plugins

cd ~/Code/rust/pluggie
cargo run -r
