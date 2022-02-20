cd ../
cargo build --release
cd deployer
python3 ./setup.py --server -u
# python3 ./run.py --server -u