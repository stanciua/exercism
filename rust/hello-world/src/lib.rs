
pub fn hello(name: Option<&str>) -> String {
    match name {
        None => "Hello, World!".to_owned(),
        Some(name) => format!("Hello, {}!", name)
    }
}
