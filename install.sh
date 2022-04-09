echo "Downloading yacs..."
git clone https://github.com/xypine/yacs.git
echo "Compiling yacs..."
cd yacs
cargo build --release
echo "Compile done."
cp target/release/yacs ../yacs.x86_64
cd ..
echo "Removing source directory..."
rm -rf yacs
echo "Yacs is now installed as 'yacs.x86_64'. Have a nice day."