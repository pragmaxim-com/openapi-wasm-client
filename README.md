## Rust playground

Fooling around, reproducing, spiking, experimenting, etc.

### Usage

```
cargo run
```

```
curl -X POST http://127.0.0.1:8082/blocks -H "Content-Type: application/json" -d '{"block_id":"frog", "height":1}'
curl -X POST http://127.0.0.1:8082/addresses -H "Content-Type: application/json" -d '{"address":"croco", "balance":2}'
```

```
curl http://127.0.0.1:8082/blocks
curl http://127.0.0.1:8082/addresses
```

```
http://127.0.0.1:8082/api-docs
```

### Building OpenAPI

```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
nvm install 20
npm install --global yarn
npm install --global @redocly/cli@latest
```