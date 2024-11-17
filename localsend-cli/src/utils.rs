use std::fmt::Debug;

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
        dbg!(&input);
        match input.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => continue,
        };
    }
}
