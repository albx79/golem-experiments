GOLEM_HOST = localhost:9006

target/wasm32-wasip1/release/fate_api.wasm: $(wildcard src/*.rs)
	cargo component build --release

deploy: src/fate-api.json target/wasm32-wasip1/release/fate_api.wasm
	golem-cli component update --component-name fate_api target/wasm32-wasip1/release/fate_api.wasm -F json | jq ".componentUrn" -r
	golem-cli component redeploy --component-name fate_api --non-interactive
	-golem-cli api-deployment delete ${GOLEM_HOST}
	-golem-cli api-definition delete --id fate-api-v1 --version `golem-cli api-definition list -F json | jq ".[0].version" -r`
	golem-cli api-definition add src/fate-api.json
	golem-cli api-deployment deploy --definition fate-api-v1/`jq < src/fate-api.json ".version" -r` --host ${GOLEM_HOST}

deploy-component:
	golem-cli component add --component-name fate_api target/wasm32-wasip1/release/fate_api.wasm

deploy-api:
	golem-cli worker add --worker-name fate-character-1 --component-name fate_api
