//! CLI do E2T: fetch-emoji-list, build, validate.

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use clap::{Parser, Subcommand};
use tokenizers::Tokenizer;

use crate::emoji_data::{codepoint_hex, read_emoji_codepoints_from_path};
use crate::emoji_words::load_emoji_words;
use crate::metrics::{validate_dataset, ValidationReport};

#[derive(Parser)]
#[command(name = "e2t")]
#[command(about = "Mapeamento emoji → token/word: dataset e métricas para tokenização de emojis")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lê emoji-data.txt e escreve lista de emojis (um por linha) em UTF-8
    #[command(name = "fetch-emoji-list")]
    FetchEmojiList {
        /// Ficheiro emoji-data.txt (Unicode UTS #51)
        #[arg(default_value = "data/emoji-data.txt")]
        input: String,
        /// Ficheiro de saída (um emoji por linha)
        #[arg(long, default_value = "data/emoji_list.txt")]
        output: String,
    },

    /// Constrói o dataset CSV: para cada emoji, tokeniza e regista n_tokens e token_ids
    #[command(name = "build")]
    Build {
        /// Ficheiro com lista de emojis (um por linha), ou diretório com emoji-data.txt
        #[arg(default_value = "data/emoji_list.txt")]
        emoji_list: String,
        /// Caminho para tokenizer.json (HuggingFace)
        #[arg(long, short = 't')]
        tokenizer: String,
        /// Identificador do tokenizer (ex.: gpt2, LLaMA-2)
        #[arg(long, default_value = "gpt2")]
        tokenizer_id: String,
        /// Ficheiro CSV de saída
        #[arg(long, default_value = "data/emoji_dictionary.csv")]
        output: String,
        /// Se emoji_list for emoji-data.txt, usar parser em vez de uma linha por emoji
        #[arg(long)]
        from_emoji_data: bool,
    },

    /// Valida o dataset CSV: estatísticas e distribuição de n_tokens
    #[command(name = "validate")]
    Validate {
        /// Ficheiro CSV do dataset
        #[arg(default_value = "data/emoji_dictionary.csv")]
        csv: String,
    },

    /// Converte frase com emojis em palavras (usa dicionário: emoji_dictionary.csv ou emoji_words.csv)
    #[command(name = "to-words")]
    ToWords {
        /// Entrada: frase com emojis, ex. "[🧒, 🍎]" ou "🧒 🍎"
        #[arg(default_value = "")]
        input: String,
        /// Ficheiro CSV do dicionário (emoji_dictionary.csv ou emoji_words.csv; usa coluna word se existir, senão token_strs)
        #[arg(long, default_value = "data/emoji_dictionary.csv")]
        dictionary: String,
        /// Ler entrada de ficheiro em vez do argumento
        #[arg(long)]
        input_file: Option<String>,
    },
}

/// Ponto de entrada da CLI. Retorna código de saída (0 = ok, não-zero = erro).
pub fn run(cli: Cli) -> i32 {
    match cli.command {
        Commands::FetchEmojiList { input, output } => cmd_fetch_emoji_list(&input, &output),
        Commands::Build {
            emoji_list,
            tokenizer,
            tokenizer_id,
            output,
            from_emoji_data,
        } => cmd_build(
            &emoji_list,
            &tokenizer,
            &tokenizer_id,
            &output,
            from_emoji_data,
        ),
        Commands::Validate { csv } => cmd_validate(&csv),
        Commands::ToWords {
            input,
            dictionary,
            input_file,
        } => cmd_to_words(&input, &dictionary, input_file.as_deref()),
    }
}

fn cmd_fetch_emoji_list(input: &str, output: &str) -> i32 {
    let path = Path::new(input);
    if !path.exists() {
        eprintln!("Ficheiro não encontrado: {}", input);
        eprintln!("Descarregue emoji-data.txt de https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt");
        return 1;
    }
    let codepoints = match read_emoji_codepoints_from_path(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Erro ao ler emoji-data: {}", e);
            return 1;
        }
    };
    let mut f = match File::create(output) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Erro ao criar {}: {}", output, e);
            return 1;
        }
    };
    for c in &codepoints {
        if writeln!(f, "{}", c).is_err() {
            eprintln!("Erro ao escrever emoji");
            return 1;
        }
    }
    eprintln!("Escritos {} emojis em {}", codepoints.len(), output);
    0
}

fn load_emoji_list(emoji_list_path: &str, from_emoji_data: bool) -> Result<Vec<char>, String> {
    let path = Path::new(emoji_list_path);
    if !path.exists() {
        return Err(format!("Ficheiro não encontrado: {}", emoji_list_path));
    }
    if from_emoji_data {
        read_emoji_codepoints_from_path(path).map_err(|e| e.to_string())
    } else {
        let f = File::open(path).map_err(|e| e.to_string())?;
        let mut chars = Vec::new();
        for line in BufReader::new(f).lines() {
            let line = line.map_err(|e| e.to_string())?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            // Uma linha = um emoji (um caractere; lista gerada por fetch-emoji-list)
            if let Some(c) = line.chars().next() {
                chars.push(c);
            }
        }
        Ok(chars)
    }
}

fn cmd_build(
    emoji_list: &str,
    tokenizer_path: &str,
    tokenizer_id: &str,
    output: &str,
    from_emoji_data: bool,
) -> i32 {
    let emojis = match load_emoji_list(emoji_list, from_emoji_data) {
        Ok(e) => e,
        Err(s) => {
            eprintln!("{}", s);
            return 1;
        }
    };
    if emojis.is_empty() {
        eprintln!("Lista de emojis vazia");
        return 1;
    }
    let tokenizer = match Tokenizer::from_file(tokenizer_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erro ao carregar tokenizer de {}: {}", tokenizer_path, e);
            eprintln!("Descarregue tokenizer.json (ex.: HuggingFace gpt2) e indique o caminho com --tokenizer");
            return 1;
        }
    };

    let mut w = csv::Writer::from_path(output).unwrap_or_else(|e| {
        eprintln!("Erro ao criar CSV {}: {}", output, e);
        std::process::exit(1);
    });
    w.write_record(&[
        "emoji",
        "codepoint_hex",
        "tokenizer_id",
        "n_tokens",
        "token_ids",
        "token_strs",
        "word",
    ])
    .expect("write header");

    for c in &emojis {
        let s: String = c.to_string();
        let encoding = tokenizer.encode(s.clone(), true).map_err(|e| e.to_string());
        let encoding = match encoding {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Erro ao tokenizar '{}': {}", s, e);
                continue;
            }
        };
        let ids: Vec<u32> = encoding.get_ids().to_vec();
        let n = ids.len() as u32;
        let token_ids_str = ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let token_strs: Vec<String> = encoding
            .get_tokens()
            .iter()
            .map(|s| s.to_string())
            .collect();
        let token_strs_str = token_strs.join(" ");

        // Nome Unicode em minúsculas (ex.: grinning face) como "word" — dicionário completo e automático
        let word = unicode_names2::name(*c)
            .map(|n| n.to_string().to_lowercase())
            .unwrap_or_default();

        w.write_record(&[
            s.clone(),
            codepoint_hex(*c),
            tokenizer_id.to_string(),
            n.to_string(),
            token_ids_str,
            token_strs_str,
            word,
        ])
        .expect("write row");
    }
    w.flush().expect("flush");
    eprintln!("Dataset escrito: {} ({} emojis)", output, emojis.len());
    0
}

fn cmd_validate(csv_path: &str) -> i32 {
    let path = Path::new(csv_path);
    if !path.exists() {
        eprintln!("Ficheiro não encontrado: {}", csv_path);
        return 1;
    }
    let (_, report): (Vec<_>, ValidationReport) = match validate_dataset(path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Erro ao validar: {}", e);
            return 1;
        }
    };
    print!("{}", report);
    0
}

/// Extrai sequência de emojis da entrada: "[🧒, 🍎]" ou "🧒 🍎" ou "🧒,🍎"
fn parse_emoji_sequence(input: &str) -> Vec<String> {
    let s = input.trim();
    let inner = if s.starts_with('[') && s.ends_with(']') {
        s[1..s.len() - 1].trim()
    } else {
        s
    };
    inner
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect()
}

fn cmd_to_words(input: &str, dictionary_path: &str, input_file: Option<&str>) -> i32 {
    let path = Path::new(dictionary_path);
    if !path.exists() {
        eprintln!("Dicionário não encontrado: {}", dictionary_path);
        eprintln!(
            "Use data/emoji_dictionary.csv (gerado por build) ou data/emoji_words.csv (emoji,word)"
        );
        return 1;
    }
    let dict = match load_emoji_words(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Erro ao carregar dicionário: {}", e);
            return 1;
        }
    };

    let input_str = if let Some(file_path) = input_file {
        match std::fs::read_to_string(file_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Erro ao ler ficheiro {}: {}", file_path, e);
                return 1;
            }
        }
    } else if input.is_empty() {
        let mut buf = String::new();
        if std::io::stdin().read_to_string(&mut buf).is_err() {
            eprintln!("Erro ao ler stdin");
            return 1;
        }
        buf
    } else {
        input.to_string()
    };

    let emojis = parse_emoji_sequence(&input_str);
    let words: Vec<&str> = emojis
        .iter()
        .map(|e| dict.get(e).map(|w| w.as_str()).unwrap_or(e.as_str()))
        .collect();
    println!("{}", words.join(" "));
    0
}
