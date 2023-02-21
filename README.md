## Run the project locally

Use trunk to run the project on ["localhost:8080"](http://localhost:8080/)
```
trunk serve
```

## Environment setup (for ubuntu)

Install Rust
```
sudo snap install --classic rustup
rustup toolchain install stable
rustup default stable
```

Install gcc
```
sudo apt update
sudo apt install gcc
```

Install Node and npm
```
sudo apt update
sudo apt install nodejs npm
```

Install WebAssembly
```
rustup update
cargo install wasm-pack
cargo install cargo-generate
sudo npm install npm@latest -g
```

Install Trunk
```
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Add VSCode extension [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)