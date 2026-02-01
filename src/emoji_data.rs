//! Parser do ficheiro Unicode emoji-data.txt (UTS #51).
//!
//! Lê apenas a primeira secção "Emoji" (codepoints e intervalos) e expande
//! para uma lista de caracteres emoji (um codepoint por entrada; emojis
//! compostos/sequências não são expandidos nesta versão).

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
/// pertence à propriedade "Emoji" (ex.: "Emoji_Presentation"), para não incluir
/// duplicados.
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
        if !line.contains(" ; Emoji ") && !line.contains(" ; Emoji\n") {
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

/// Formata um `char` como codepoint Unicode em hexadecimal (ex.: "1F600", "0023").
/// Codepoints &lt; 0x10000 são formatados com 4 dígitos; maiores com 5–6 dígitos.
pub fn codepoint_hex(c: char) -> String {
    let u = c as u32;
    if u <= 0xFFFF {
        format!("{:04X}", u)
    } else {
        format!("{:X}", u)
    }
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
}
