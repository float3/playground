#!/usr/bin/env bash
cargo fmt --all

cargo fix --allow-dirty --allow-staged
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features -- -D warnings

cargo check --release
cargo check

cargo test
cargo test --release

./build.sh
cd ./ts

curl https://raw.githubusercontent.com/float3/float3.github.io/master/static/styles.css -o ./src/styles.css

npm update
npm install
npm audit fix
npx tsc
npx eslint . --ext .ts,.tsx --fix
