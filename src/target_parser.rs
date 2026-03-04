use aviutl2::tracing;

#[cached::proc_macro::cached(result, key = "String", convert = r#"{ target.to_string() }"#)]
pub fn compile_target(target: &str) -> aviutl2::AnyResult<regex::Regex> {
    let chars = target.chars().collect::<Vec<_>>();
    let mut i = 0usize;
    let mut regexes = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c == '[' && chars.get(i + 1).copied() == Some('[') {
            // [[A-Z]]：`A`から`Z`までの範囲
            let (Some(char_start), true, Some(char_end), true, true) = (
                chars.get(i + 2).copied(),
                chars.get(i + 3).copied() == Some('-'),
                chars.get(i + 4).copied(),
                chars.get(i + 5).copied() == Some(']'),
                chars.get(i + 6).copied() == Some(']'),
            ) else {
                return Err(aviutl2::anyhow::anyhow!(
                    "Invalid range format at position {}",
                    i
                ));
            };
            if char_start > char_end {
                return Err(aviutl2::anyhow::anyhow!(
                    "Invalid range: start character '{}' is greater than end character '{}'",
                    char_start,
                    char_end
                ));
            }
            let regex_str = format!(
                r"[{}-{}]",
                regex::escape(&char_start.to_string()),
                regex::escape(&char_end.to_string())
            );
            regexes.push(regex_str);
            i += 7;
        } else if c == '{' && chars.get(i + 1).copied() == Some('{') {
            // {{...}}：`\p{...}`相当
            let end_pos = chars[i + 2..]
                .windows(2)
                .position(|w| w == ['}', '}'])
                .ok_or_else(|| {
                    aviutl2::anyhow::anyhow!("Unclosed group starting at position {}", i)
                })?;
            let group_name: String = chars[i + 2..i + 2 + end_pos].iter().collect();
            let regex_str = format!(r"\p{{{}}}", group_name);
            regexes.push(regex_str);
            i += 4 + end_pos; // 2 for '{{', 2 for '}}'
        } else {
            regexes.push(regex::escape(&c.to_string()));
            i += 1;
        }
    }

    let combined_regex = regexes.join("|");

    tracing::debug!("Compiled target '{}' into regex '{}'", target, combined_regex);
    regex::Regex::new(&combined_regex).map_err(|e| {
        aviutl2::anyhow::anyhow!("Failed to compile regex '{}': {}", combined_regex, e)
    })
}
