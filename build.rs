use prost_build::Config;

fn main() {
    let mut config = Config::new();
    config.type_attribute(".", "#[derive(Eq)]");
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config.type_attribute(".", "#[derive(Ord)]");
    config
        .compile_protos(&["./src/example.proto"], &["./src"])
        .unwrap();
}
