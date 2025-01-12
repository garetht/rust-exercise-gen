From within frontend directory

`protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --ts_proto_out=. --proto_path=.. ../src/protos/exercises.proto`
