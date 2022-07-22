fn main() {
    cc::Build::new()
        .cpp(true)
        .include(".")
        .include(std::env::var("DEP_LUA_INCLUDE").unwrap())
        .file("yuescript/ast.cpp")
        .file("yuescript/parser.cpp")
        .file("yuescript/yue_compiler.cpp")
        .file("yuescript/yue_parser.cpp")
        .file("yuescript/yuescript.cpp")
        .flag_if_supported("-std=c++17")
        .compile("yue");

    println!("cargo:rerun-if-changed=build.rs");
}
