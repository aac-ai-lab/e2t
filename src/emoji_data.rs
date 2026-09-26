//! Parser do ficheiro Unicode emoji-data.txt (UTS #51) e sequências
//! (emoji-zwj-sequences.txt / emoji-sequences.txt).
//!
//! - **Codepoints:** primeira secção "Emoji" de `emoji-data.txt` (um `char` por entrada).
//! - **Sequências:** linhas `CP1 CP2 … ; Tipo ; …` em ficheiros de sequências Unicode
//!   (ZWJ, bandeiras, modificadores, etc.), devolvidas como `String` UTF-8.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmojiDataError {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Linha inválida (esperado codepoint ou intervalo): {0}")]
    InvalidLine(String),
    #[error("Codepoint fora do intervalo Unicode: {0}")]
    InvalidCodepoint(u32),
}

/// Lê `emoji-data.txt` e devolve todos os caracteres emoji da primeira secção "Emoji".
/// Para cada linha da forma `XXXX ; Emoji` ou `XXXX..YYYY ; Emoji`, expande os
/// codepoints e converte para `char`. Para quando encontra uma linha que não
/// pertence à propriedade "Emoji" (ex.: "Emoji_Presentation").
pub fn read_emoji_codepoints_from_path(path: &Path) -> Result<Vec<char>, EmojiDataError> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut codepoints = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Parar na próxima secção (ex.: "Emoji_Presentation")
        if line.contains("Emoji_Presentation") || line.contains("Emoji_Modifier") {
            break;
        }
        if !line.contains(" ; Emoji ") && !line.ends_with(" ; Emoji") {
            continue;
        }
        let part = line
            .split(" ; ")
            .next()
            .ok_or_else(|| EmojiDataError::InvalidLine(line.to_string()))?
            .trim();
        if part.contains("..") {
            let mut split = part.split("..");
            let low = split
                .next()
                .ok_or_else(|| EmojiDataError::InvalidLine(line.to_string()))?;
            let high = split
                .next()
                .ok_or_else(|| EmojiDataError::InvalidLine(line.to_string()))?;
            let low_u = u32::from_str_radix(low, 16)
                .map_err(|_| EmojiDataError::InvalidLine(line.to_string()))?;
            let high_u = u32::from_str_radix(high, 16)
                .map_err(|_| EmojiDataError::InvalidLine(line.to_string()))?;
            for cp in low_u..=high_u {
                if let Some(c) = char::from_u32(cp) {
                    codepoints.push(c);
                }
            }
        } else {
            let cp = u32::from_str_radix(part, 16)
                .map_err(|_| EmojiDataError::InvalidLine(line.to_string()))?;
            if let Some(c) = char::from_u32(cp) {
                codepoints.push(c);
            } else {
                return Err(EmojiDataError::InvalidCodepoint(cp));
            }
        }
    }
    Ok(codepoints)
}

/// Lê `emoji-zwj-sequences.txt` ou `emoji-sequences.txt` (UTS #51).
///
/// Formato típico:
/// `1F468 200D 1F469 200D 1F466 ; RGI_Emoji_ZWJ_Sequence ; … # … (👨‍👩‍👦)`
pub fn read_emoji_sequences_from_path(path: &Path) -> Result<Vec<String>, EmojiDataError> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut sequences = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let code_part = line
            .split(';')
            .next()
            .ok_or_else(|| EmojiDataError::InvalidLine(line.to_string()))?
            .trim();
        if code_part.is_empty() {
            continue;
        }
        let mut chars = String::new();
        for tok in code_part.split_whitespace() {
            let cp = u32::from_str_radix(tok, 16)
                .map_err(|_| EmojiDataError::InvalidLine(line.to_string()))?;
            if let Some(c) = char::from_u32(cp) {
                chars.push(c);
            } else {
                return Err(EmojiDataError::InvalidCodepoint(cp));
            }
        }
        if !chars.is_empty() {
            sequences.push(chars);
        }
    }
    Ok(sequences)
}

/// Formata um `char` como codepoint Unicode em hexadecimal (ex.: "1F600", "0023").
pub fn codepoint_hex(c: char) -> String {
    let u = c as u32;
    if u <= 0xFFFF {
        format!("{:04X}", u)
    } else {
        format!("{:X}", u)
    }
}

/// Hex de um emoji (um codepoint ou sequência): codepoints separados por espaço.
pub fn emoji_hex(s: &str) -> String {
    s.chars().map(codepoint_hex).collect::<Vec<_>>().join(" ")
}

/// Nome inglês aproximado: um codepoint via `unicode_names2`; sequências = nomes unidos.
pub fn word_en_for_emoji(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() == 1 {
        return unicode_names2::name(chars[0])
            .map(|n| n.to_string().to_lowercase())
            .unwrap_or_default();
    }
    chars
        .into_iter()
        .filter_map(|c| {
            // ZWJ e VS-16 não entram no nome composto
            if c == '\u{200D}' || c == '\u{FE0F}' {
                return None;
            }
            unicode_names2::name(c).map(|n| n.to_string().to_lowercase())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_parse_single_and_range() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("emoji-data.txt");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "# comment").unwrap();
        writeln!(f, "0023 ; Emoji # E0.0 [#] hash").unwrap();
        writeln!(f, "0030..0032 ; Emoji # E0.0 [3] digits").unwrap();
        writeln!(f, "1F600 ; Emoji # E1.0 [1] grinning").unwrap();
        writeln!(f, "231A..231B ; Emoji_Presentation # other").unwrap();
        f.sync_all().unwrap();
        drop(f);

        let chars = read_emoji_codepoints_from_path(&path).unwrap();
        assert_eq!(chars.len(), 1 + 3 + 1); // 0023, 0030,0031,0032, 1F600
        assert_eq!(chars[0], '#');
        assert_eq!(chars[1], '0');
        assert_eq!(chars[2], '1');
        assert_eq!(chars[3], '2');
        assert_eq!(chars[4], '\u{1F600}');
    }

    #[test]
    fn test_codepoint_hex() {
        assert_eq!(codepoint_hex('😀'), "1F600");
        assert_eq!(codepoint_hex('#'), "0023");
    }

    #[test]
    fn test_parse_zwj_sequence() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("emoji-zwj-sequences.txt");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "# comment").unwrap();
        writeln!(
            f,
            "1F468 200D 1F469 200D 1F466 ; RGI_Emoji_ZWJ_Sequence ; family: man, woman, boy # E2.0 [1] (👨‍👩‍👦)"
        )
        .unwrap();
        f.sync_all().unwrap();
        drop(f);

        let seqs = read_emoji_sequences_from_path(&path).unwrap();
        assert_eq!(seqs.len(), 1);
        assert_eq!(seqs[0], "👨‍👩‍👦");
        assert_eq!(emoji_hex(&seqs[0]), "1F468 200D 1F469 200D 1F466");
    }
}
