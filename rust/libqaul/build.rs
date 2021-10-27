use std::env;
use std::fs;
use std::path::Path;

fn main() {
	prost_build::compile_protos(
		&[
		"rpc/qaul_rpc.proto",
		"node/node.proto",
		"node/user_accounts.proto",
		"router/router.proto",
		"services/feed/feed.proto",
		"connections/ble/manager/ble.proto",
		], 
		&[
			"src"
		]
	).unwrap();

	// copy generated protobuf files to their module locations
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let to = Path::new("src/rpc/protobuf_generated/rust");

	// copy to central rust file folder
	fs::copy(Path::new(&out_dir).join("qaul.rpc.rs"), to.join("qaul.rpc.rs")).unwrap();
	fs::copy(Path::new(&out_dir).join("qaul.rpc.node.rs"), to.join("qaul.rpc.node.rs")).unwrap();
	fs::copy(Path::new(&out_dir).join("qaul.rpc.user_accounts.rs"), to.join("qaul.rpc.user_accounts.rs")).unwrap();
	fs::copy(Path::new(&out_dir).join("qaul.rpc.router.rs"), to.join("qaul.rpc.router.rs")).unwrap();
	fs::copy(Path::new(&out_dir).join("qaul.rpc.feed.rs"), to.join("qaul.rpc.feed.rs")).unwrap();
	fs::copy(Path::new(&out_dir).join("qaul.sys.ble.rs"), to.join("qaul.sys.ble.rs")).unwrap();

	// copy to modules
	fs::copy(&Path::new(&out_dir).join("qaul.rpc.rs"), Path::new("src/rpc/qaul.rpc.rs")).unwrap();
	fs::copy(&Path::new(&out_dir).join("qaul.rpc.node.rs"), Path::new("src/node/qaul.rpc.node.rs")).unwrap();
	fs::copy(&Path::new(&out_dir).join("qaul.rpc.user_accounts.rs"), Path::new("src/node/qaul.rpc.user_accounts.rs")).unwrap();
	fs::copy(&Path::new(&out_dir).join("qaul.rpc.router.rs"), Path::new("src/router/qaul.rpc.router.rs")).unwrap();
	fs::copy(&Path::new(&out_dir).join("qaul.rpc.feed.rs"), Path::new("src/services/feed/qaul.rpc.feed.rs")).unwrap();
	fs::copy(&Path::new(&out_dir).join("qaul.sys.ble.rs"), Path::new("src/connections/ble/manager/qaul.sys.ble.rs")).unwrap();
}
