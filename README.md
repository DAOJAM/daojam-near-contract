### Build the contract
bash:
```bash
export RUSTFLAGS='-C link-arg=-s';
cargo build --target wasm32-unknown-unknown --release
```
cmd:
```cmd
set RUSTFLAGS=-C link-arg=-s
cargo build --target wasm32-unknown-unknown --release
```
powershell:
```powershell
$env:RUSTFLAGS='-C link-arg=-s'
cargo build --target wasm32-unknown-unknown --release
```

### deploy the contract
```
near deploy --accountId <accountId> --wasmFile <file>
```

