cargo build --release --target wasm32-unknown-unknown --package todos_backend

candid-extractor target/wasm32-unknown-unknown/release/todos_backend.wasm >src/todos_backend/todos_backend.did