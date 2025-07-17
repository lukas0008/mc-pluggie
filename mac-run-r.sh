cargo build -r -p mc-network --features mc-network/init &
cargo build -r -p mc-tick --features mc-tick/init &
cargo build -r -p mc-example --features mc-example/init &
cargo build -r -p mc-loader --features mc-loader/init &
cargo build -r -p mc-status --features mc-status/init &
cargo build -r -p mc-registry --features mc-registry/init &
cargo build -r -p mc-keepalive --features mc-keepalive/init &

wait

cp ./target/release/libmc_network.dylib ../pluggie/plugins/
cp ./target/release/libmc_tick.dylib ../pluggie/plugins
cp ./target/release/libmc_example.dylib ../pluggie/plugins
cp ./target/release/libmc_loader.dylib ../pluggie/plugins
cp ./target/release/libmc_status.dylib ../pluggie/plugins
cp ./target/release/libmc_registry.dylib ../pluggie/plugins
cp ./target/release/libmc_keepalive.dylib ../pluggie/plugins

cd ~/Code/rust/pluggie
cargo run -r
