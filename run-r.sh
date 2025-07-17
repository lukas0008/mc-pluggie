cargo build -r -p mc-network --features mc-network/init
cargo build -r -p mc-tick --features mc-tick/init
cargo build -r -p mc-example --features mc-example/init
cargo build -r -p mc-loader --features mc-loader/init
cargo build -r -p mc-status --features mc-status/init
cargo build -r -p mc-registry --features mc-registry/init

cp ./target/release/libmc_network.so ../pluggie/plugins
cp ./target/release/libmc_tick.so ../pluggie/plugins
cp ./target/release/libmc_example.so ../pluggie/plugins
cp ./target/release/libmc_loader.so ../pluggie/plugins
cp ./target/release/libmc_status.so ../pluggie/plugins
cp ./target/release/libmc_registry.so ../pluggie/plugins

cd ~/Code/rust/pluggie
cargo run -r
