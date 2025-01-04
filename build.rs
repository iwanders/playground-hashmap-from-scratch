fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=src/std_unordered_map.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/std_unordered_map.cpp")
        .compile("std_unordered_map");
}
