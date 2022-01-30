## Prequisite
- install camke [https://cmake.org/download/]


## Generate Proto Message struct from protofile
Look at https ://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen for more information

```
cargo install protobuf-codegen
set PATH="$HOME/.cargo/bin:$PATH"
run protoc --rust_out . GenericMessage.proto
```
