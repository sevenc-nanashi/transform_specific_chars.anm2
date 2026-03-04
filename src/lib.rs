use aviutl2::module::ScriptModuleFunctions;

mod target_parser;
mod text_parser;

#[aviutl2::plugin(ScriptModule)]
struct TransformSpecificCharsMod2 {}

impl aviutl2::module::ScriptModule for TransformSpecificCharsMod2 {
    fn new(_info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        aviutl2::tracing_subscriber::fmt()
            .with_max_level(if cfg!(debug_assertions) {
                aviutl2::tracing::Level::DEBUG
            } else {
                aviutl2::tracing::Level::INFO
            })
            .event_format(aviutl2::logger::AviUtl2Formatter)
            .with_writer(aviutl2::logger::AviUtl2LogWriter)
            .init();

        Ok(Self {})
    }
    fn plugin_info(&self) -> aviutl2::module::ScriptModuleTable {
        aviutl2::module::ScriptModuleTable {
            information: "transform_specific_chars.anm2 / Internal Module".into(),
            functions: Self::functions(),
        }
    }
}

#[cached::proc_macro::cached(result)]
fn is_target_char_simple(
    index: usize,
    text: String,
    target_chars: String,
) -> aviutl2::AnyResult<bool> {
    if target_chars.is_empty() {
        return Ok(false);
    }
    let chars = crate::text_parser::parse_text(&text)?;
    let char = chars
        .chars()
        .nth(index)
        .ok_or_else(|| aviutl2::anyhow::anyhow!("Index {} is out of bounds", index))?;
    let target_regex = crate::target_parser::compile_target(&target_chars)?;
    Ok(target_regex.is_match(&char.to_string()))
}

#[cached::proc_macro::cached(result)]
fn matched_ranges(text: String, regex: String) -> aviutl2::AnyResult<Vec<(usize, usize)>> {
    let chars = crate::text_parser::parse_text(&text)?;
    let target_regex = regex::Regex::new(&regex)?;

    // NOTE: m.start()、m.end()はバイトインデックスなので、文字インデックスに変換する必要がある
    let mut char_indices = chars
        .char_indices()
        .enumerate()
        .map(|(i, (byte_index, _))| (byte_index, i))
        .collect::<std::collections::BTreeMap<usize, usize>>();
    char_indices.insert(chars.len(), chars.chars().count()); // テキストの終端もマップに追加
                                                             //
    Ok(target_regex
        .find_iter(&chars)
        .map(|m| (char_indices[&m.start()], char_indices[&m.end()]))
        .collect())
}

#[aviutl2::module::functions]
impl TransformSpecificCharsMod2 {
    fn is_target_char(
        &self,
        index: usize,
        regex: bool,
        text: String,
        target_chars: String,
    ) -> aviutl2::AnyResult<bool> {
        if target_chars.is_empty() {
            return Ok(false);
        }

        if regex {
            let ranges = matched_ranges(text, target_chars)?;
            Ok(ranges
                .into_iter()
                .any(|(start, end)| index >= start && index < end))
        } else {
            is_target_char_simple(index, text, target_chars)
        }
    }
}

aviutl2::register_script_module!(TransformSpecificCharsMod2);
