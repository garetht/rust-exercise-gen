To generate Rust

`protoc --rs_out ./src/protos/ src/protos/exercises.proto`

From within frontend directory, to generate

`protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=. --proto_path=.. ../src/protos/exercises.proto`
