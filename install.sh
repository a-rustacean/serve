cargo build --release
rm -rf $HOME/.local/bin/serve
ln -s $(pwd)/target/release/serve $HOME/.local/bin/serve
chmod u+x $HOME/.local/bin/serve
