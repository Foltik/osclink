fn main() {
	prost_build::compile_protos(&["src/net/link/mod.proto"], &["src/net/link/"]).unwrap();
}
