//! Dicionário emoji → palavra (CSV).
//!
//! Suporta dois formatos:
//! - **emoji_words.csv:** colunas `emoji`, `word`
//! - **emoji_dictionary.csv:** colunas `emoji`, …, `token_strs`, e opcionalmente `word`.
//!   Se existir coluna `word` preenchida, usa-se; senão usa-se `token_strs`.

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use csv::ReaderBuilder;

/// Carrega o dicionário emoji → string a partir de um CSV.
/// Aceita:
/// - CSV com colunas `emoji` e `word` (ex.: emoji_words.csv)
/// - CSV com colunas `emoji` e `token_strs`, e opcionalmente `word` (ex.: emoji_dictionary.csv).
///   Se a linha tiver `word` não vazio, usa `word`; senão usa `token_strs`.
pub fn load_emoji_words(path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
    let f = File::open(path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(f);
    let headers = reader
        .headers()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
        .clone();
    let emoji_idx = headers
        .iter()
        .position(|h| h.trim().eq_ignore_ascii_case("emoji"))
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "CSV sem coluna 'emoji'")
        })?;
    let word_idx = headers
        .iter()
        .position(|h| h.trim().eq_ignore_ascii_case("word"));
    let token_strs_idx = headers
        .iter()
        .position(|h| h.trim().eq_ignore_ascii_case("token_strs"));

    let mut map = HashMap::new();
    for result in reader.records() {
        let record = result.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let emoji = record.get(emoji_idx).unwrap_or_default().trim().to_string();
        if emoji.is_empty() {
            continue;
        }
        let value = word_idx
            .and_then(|i| record.get(i))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                token_strs_idx
                    .and_then(|i| record.get(i))
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| emoji.clone());
        map.insert(emoji, value);
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_load_emoji_words() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("emoji_words.csv");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "emoji,word").unwrap();
        writeln!(f, "🧒,CRIANÇA").unwrap();
        writeln!(f, "🍎,MAÇÃ").unwrap();
        f.sync_all().unwrap();
        drop(f);

        let dict = load_emoji_words(&path).unwrap();
        assert_eq!(dict.get("🧒"), Some(&"CRIANÇA".to_string()));
        assert_eq!(dict.get("🍎"), Some(&"MAÇÃ".to_string()));
    }

    #[test]
    fn test_load_token_dataset_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("emoji_dictionary.csv");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(
            f,
            "emoji,codepoint_hex,tokenizer_id,n_tokens,token_ids,token_strs"
        )
        .unwrap();
        writeln!(f, "🧒,1F9D2,gpt2,3,8582 100 240,\"ðŁ § Ĵ\"").unwrap();
        writeln!(f, "🍎,1F34E,gpt2,3,8582 235 236,\"ðŁ į İ\"").unwrap();
        f.sync_all().unwrap();
        drop(f);

        let dict = load_emoji_words(&path).unwrap();
        assert_eq!(dict.get("🧒"), Some(&"ðŁ § Ĵ".to_string()));
        assert_eq!(dict.get("🍎"), Some(&"ðŁ į İ".to_string()));
    }
}
