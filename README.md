## Rest-api with wasm client library

Actix-web rest-api with 2 generated client libraries from OpenAPI specification :
 - [progenitor](https://github.com/oxidecomputer/progenitor) 
 - [openapi-generator](https://github.com/OpenAPITools/openapi-generator)

and one hand-written `client` using shared `model` with the server.

### Building OpenAPI

First we build our `openapi.json` specification which will later be served at http://127.0.0.1:8082/swagger :

```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
nvm install 20
npm install --global yarn
npm install --global @redocly/cli@latest

yarn bundle
```

### Hand made conditional-client with reqwest and gloo-net 

`conditional-client` is a hand-written client with [wasm-pack](https://github.com/rustwasm/wasm-pack) generated javascript.

```
# install web-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

cd conditional-client # native target is built in root
wasm-pack build --target web
```

### Generating web client with Progenitor

```
cd progenitor
wasm-pack build --target web
```

### Generating web client with OpenAPI generator

Advantage of OpenAPI generator is that it can generate even tests for the endpoints.

```
cd openapi-gen

java -jar openapi-generator-cli.jar generate \
    -i ../openapi.json \
    -g rust \
    -o ./rust \
    --additional-properties=packageName=openapi-gen,library=reqwest

cd rust
wasm-pack build --target web
```

### Running Actix-web http server

Server handles requests from all 3 types of clients, handwritten, progenitor and openapi-gen : 
```
.service(fs::Files::new("/client", "./client").index_file("index.html"))
.service(fs::Files::new("/progenitor", "./progenitor").index_file("index.html"))
.service(fs::Files::new("/openapi-gen", "./openapi-gen/rust").index_file("index.html"))
```

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

Now you can access : 
  - swagger at http://127.0.0.1:8082/swagger 
  - hand-written client demo at http://127.0.0.1:8082/client
  - progenitor demo at http://127.0.0.1:8082/progenitor
  - openapi-gen demo at http://127.0.0.1:8082/openapi-gen
