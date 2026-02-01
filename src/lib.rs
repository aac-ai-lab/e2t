//! # E2T Emoji para Token/Word — mapeamento emoji → token
//!
//! Experimento científico que mapeia **todos os emojis** (Unicode, propriedade Emoji)
//! para a sua representação em **tokens** (por tokenizador) e opcionalmente em
//! **palavra** (nome Unicode/CLDR).
//!
//! - **Fonte de emojis:** Unicode [emoji-data.txt](https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt) (UTS #51), primeira secção "Emoji".
//! - **Tokenização:** Suporta tokenizadores compatíveis com HuggingFace (ficheiro `tokenizer.json`).
//! - **Saída:** CSV com colunas emoji, codepoint_hex, tokenizer_id, n_tokens, token_ids, token_strs.
//!
//! Ver [docs/pt-br/EXPERIMENTO_PESQUISA](docs/pt-br/EXPERIMENTO_PESQUISA.md) para enquadramento do experimento.

pub mod cli;
pub mod emoji_data;
pub mod metrics;

pub use emoji_data::{read_emoji_codepoints_from_path, codepoint_hex, EmojiDataError};
pub use metrics::validate_dataset;
pub use cli::run;
