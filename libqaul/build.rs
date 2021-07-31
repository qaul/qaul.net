fn main() {
	prost_build::compile_protos(
		&[
		"src/rpc/protobuf_definition/qaul_rpc.proto",
		"src/rpc/protobuf_definition/from_libqaul.proto",
		"src/rpc/protobuf_definition/to_libqaul.proto"
		], 
		&[
			"src/rpc/protobuf_definition"
		]
	).unwrap();
}
