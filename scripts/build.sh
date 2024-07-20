cargo build
cargo doc
rm -rf ./docs
mkdir -p ./docs
cp -r ./target/doc/crab_rocket/* ./docs