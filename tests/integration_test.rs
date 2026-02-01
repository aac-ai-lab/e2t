//! Testes de integração: fetch-emoji-list em fixtures, validate em CSV de amostra.

use std::path::Path;

use e2t::emoji_data::read_emoji_codepoints_from_path;
use e2t::metrics::validate_dataset;

#[test]
fn test_fetch_emoji_list_fixture() {
    let path = Path::new("tests/fixtures/emoji-data-sample.txt");
    if !path.exists() {
        eprintln!("Fixture não encontrado: {:?}", path);
        return;
    }
    let chars = read_emoji_codepoints_from_path(path).expect("parse emoji-data-sample");
    // 0023, 0030, 0031, 1F600, 1F601 = 5 emojis (paramos em Emoji_Presentation)
    assert_eq!(chars.len(), 5);
    assert_eq!(chars[0], '#');
    assert_eq!(chars[3], '\u{1F600}');
}

#[test]
fn test_validate_sample_csv() {
    let path = Path::new("data/emoji_token_dataset_sample.csv");
    if !path.exists() {
        eprintln!("CSV de amostra não encontrado: {:?}", path);
        return;
    }
    let (rows, report) = validate_dataset(path).expect("validate_dataset");
    assert_eq!(rows.len(), 4);
    assert_eq!(report.total_rows, 4);
    assert!(report.mean_tokens_per_emoji >= 1.0);
}
