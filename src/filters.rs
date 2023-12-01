const SUFFIX: &str = "Philippe Loctaux";

pub fn title<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let prefix = s.to_string();

    Ok(if prefix != SUFFIX {
        format!("{prefix} - {SUFFIX}")
    } else {
        prefix
    })
}
