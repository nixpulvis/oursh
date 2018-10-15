pub fn complete(text: &str) -> String {
    match text {
        "l"  => "ls".into(),
        "la" => "ls -la".into(),
        _ => "".into(),
    }
}
