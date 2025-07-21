fn report(label: &str, err: impl std::fmt::Display) {
    eprintln!("--- toml {} ---\n{}\n", label, err);
}

fn main() {
    let toml_src = r#"
[tool.uv.sources]
python-multipart = "https://example.com/foo.whl"
python-multipart = "https://example.com/bar.whl"
"#;

    // 0.8.19 behavior
    match toml8::from_str::<toml8::Value>(toml_src) {
        Ok(_) => println!("toml 0.8.19 parsed!"),
        Err(e) => report("0.8.19", e),
    }

    // 0.9.2 behavior
    match toml9::from_str::<toml9::Value>(toml_src) {
        Ok(_) => println!("toml 0.9.2 parsed!"),
        Err(e) => report("0.9.2", e),
    }
}
