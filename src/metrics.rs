//! Métricas e validação do dataset emoji → token.
//!
//! Estatísticas: distribuição de n_tokens por emoji, proporção 1-token vs multi-token,
//! comprimento médio em tokens, etc.

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmojiTokenRow {
    pub emoji: String,
    pub codepoint_hex: String,
    pub tokenizer_id: String,
    pub n_tokens: u32,
    #[serde(rename = "token_ids")]
    pub token_ids: String,
    #[serde(rename = "token_strs")]
    pub token_strs: Option<String>,
}

/// Lê o CSV do dataset e devolve as linhas e um resumo de métricas.
pub fn validate_dataset(path: &Path) -> Result<(Vec<EmojiTokenRow>, ValidationReport), std::io::Error> {
    let f = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(f);
    let mut rows = Vec::new();
    let mut n_tokens_dist: HashMap<u32, u64> = HashMap::new();
    let mut total_tokens: u64 = 0;
    let mut by_tokenizer: HashMap<String, (u64, u64)> = HashMap::new(); // (count, sum_tokens)

    for result in reader.deserialize() {
        let row: EmojiTokenRow = result.map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        })?;
        *n_tokens_dist.entry(row.n_tokens).or_insert(0) += 1;
        total_tokens += row.n_tokens as u64;
        let ent = by_tokenizer
            .entry(row.tokenizer_id.clone())
            .or_insert((0, 0));
        ent.0 += 1;
        ent.1 += row.n_tokens as u64;
        rows.push(row);
    }

    let n = rows.len() as u64;
    let mean_tokens = if n > 0 {
        total_tokens as f64 / n as f64
    } else {
        0.0
    };
    let single_token = n_tokens_dist.get(&1).copied().unwrap_or(0);
    let multi_token = n - single_token;
    let pct_single = if n > 0 {
        100.0 * single_token as f64 / n as f64
    } else {
        0.0
    };

    let report = ValidationReport {
        total_rows: n,
        mean_tokens_per_emoji: mean_tokens,
        single_token_count: single_token,
        multi_token_count: multi_token,
        pct_single_token: pct_single,
        n_tokens_distribution: n_tokens_dist,
        by_tokenizer,
    };

    Ok((rows, report))
}

#[derive(Debug)]
pub struct ValidationReport {
    pub total_rows: u64,
    pub mean_tokens_per_emoji: f64,
    pub single_token_count: u64,
    pub multi_token_count: u64,
    pub pct_single_token: f64,
    pub n_tokens_distribution: HashMap<u32, u64>,
    pub by_tokenizer: HashMap<String, (u64, u64)>,
}

impl std::fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== E2T Validação do dataset emoji → token ===")?;
        writeln!(f, "Total de linhas: {}", self.total_rows)?;
        writeln!(f, "Média de tokens por emoji: {:.4}", self.mean_tokens_per_emoji)?;
        writeln!(f, "Emojis em 1 token: {} ({:.2}%)", self.single_token_count, self.pct_single_token)?;
        writeln!(f, "Emojis em >1 token: {}", self.multi_token_count)?;
        writeln!(f, "")?;
        writeln!(f, "Distribuição de n_tokens (amostra):")?;
        let mut dist_vec: Vec<_> = self.n_tokens_distribution.iter().collect();
        dist_vec.sort_by_key(|(k, _)| *k);
        for (k, v) in dist_vec.iter().take(15) {
            writeln!(f, "  {} token(s): {}", k, v)?;
        }
        if dist_vec.len() > 15 {
            writeln!(f, "  ... ({} valores distintos)", dist_vec.len())?;
        }
        writeln!(f, "")?;
        writeln!(f, "Por tokenizer:")?;
        for (tid, (count, sum_tok)) in &self.by_tokenizer {
            let mean = if *count > 0 {
                *sum_tok as f64 / *count as f64
            } else {
                0.0
            };
            writeln!(f, "  {}: {} linhas, média {:.4} tokens/emoji", tid, count, mean)?;
        }
        Ok(())
    }
}
