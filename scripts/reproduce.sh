#!/usr/bin/env bash
# Script de reprodução: obtém lista de emojis, constrói dataset e valida.
# Uso: ./scripts/reproduce.sh [emoji_data_path] [tokenizer_path] [csv_out]
#   emoji_data_path: emoji-data.txt (default: data/emoji-data.txt)
#   tokenizer_path: tokenizer.json (obrigatório para build; sem ele só fetch + validate em amostra)
#   csv_out: CSV de saída (default: data/emoji_token_dataset.csv)

set -e

EMOJI_DATA="${1:-data/emoji-data.txt}"
TOKENIZER="${2:-}"
CSV_OUT="${3:-data/emoji_token_dataset.csv}"
BIN="target/release/e2t"

echo "=== E2T Emoji para Token/Word — Reprodução ==="
echo "Emoji data: $EMOJI_DATA"
echo "Tokenizer:  ${TOKENIZER:-<não indicado>}"
echo "CSV saída:  $CSV_OUT"
echo ""

if ! cargo build --release 2>/dev/null; then
    echo "A compilar..."
    cargo build --release
fi

if [[ -f "$EMOJI_DATA" ]]; then
    echo "1. Gerar lista de emojis a partir de $EMOJI_DATA"
    "$BIN" fetch-emoji-list "$EMOJI_DATA" --output data/emoji_list.txt
    echo ""
else
    echo "Aviso: $EMOJI_DATA não encontrado. Descarregue de https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt"
    echo "A usar fixtures para teste (fetch a partir de tests/fixtures/emoji-data-sample.txt)."
    "$BIN" fetch-emoji-list tests/fixtures/emoji-data-sample.txt --output /tmp/emoji_list_fixtures.txt
    EMOJI_LIST="/tmp/emoji_list_fixtures.txt"
    CSV_OUT="/tmp/emoji_token_fixtures.csv"
fi

if [[ -n "$TOKENIZER" && -f "$TOKENIZER" ]]; then
    echo "2. Construir dataset → $CSV_OUT"
    "$BIN" build "${EMOJI_LIST:-data/emoji_list.txt}" --tokenizer "$TOKENIZER" --tokenizer-id gpt2 --output "$CSV_OUT"
    echo ""
    echo "3. Validar dataset"
    "$BIN" validate "$CSV_OUT"
else
    echo "Tokenizer não indicado ou ficheiro inexistente. A saltar build e validate do dataset completo."
    echo "Para build: ./scripts/reproduce.sh $EMOJI_DATA /caminho/para/tokenizer.json $CSV_OUT"
    if [[ -f data/emoji_token_dataset_sample.csv ]]; then
        echo "Validar CSV de amostra:"
        "$BIN" validate data/emoji_token_dataset_sample.csv
    fi
fi

echo ""
echo "=== Fim da reprodução ==="
