fn main() {
    ::capnpc::CompilerCommand::new()
        .file("addressbook.capnp")
        .edition(::capnpc::codegen::RustEdition::Rust2018)
        .run()
        .expect("compiling schema");
}
