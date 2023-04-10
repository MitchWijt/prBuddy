cargo build --release &&
cd target/release &&
tar -czf prbuddy-mac.tar.gz pr_buddy &&
shasum -a 256 prbuddy-mac.tar.gz