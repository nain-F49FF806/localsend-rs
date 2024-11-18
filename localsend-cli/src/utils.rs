use std::{fmt::Debug, path::PathBuf};

use path_clean::PathClean;

pub fn dbgr<V>(value: &V)
where
    V: Debug,
{
    dbg!(value);
}

/// Present query to user, and wait for confirmation (y/n)
/// If default provided, will highlight that option
pub fn ask_confirm(query: &str, default: Option<bool>) -> Result<bool, std::io::Error> {
    let options_text = if let Some(default_yes) = default {
        if default_yes {
            "(Y/n)"
        } else {
            "(y/N)"
        }
    } else {
        "(y/n)"
    };
    loop {
        println!("{} {}", query, options_text);
        let mut buffer = String::new();

        // `read_line` returns `Result` of bytes read
        std::io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim_end().to_lowercase();
        match input.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => continue,
        };
    }
}

/// returns relative path cleaned of any dots (./ ../ ../../) or leading root (/)
pub fn sanitize_relative_path(file_path: &str) -> PathBuf {
    // process filename path, first clean (using path clean) and then remove leading /
    let clean_path = PathBuf::from(file_path).clean();
    let relative_path = if clean_path.starts_with("/") {
        clean_path
            .strip_prefix("/")
            .expect("checked for starts, this shouldn't error")
    } else {
        &clean_path
    };
    relative_path.to_path_buf()
}
