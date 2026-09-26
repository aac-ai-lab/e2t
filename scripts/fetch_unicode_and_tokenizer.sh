#!/usr/bin/env bash
# Descarrega dados Unicode e o tokenizer GPT-2 usados pelo pipeline E2T.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DATA="$ROOT/data"
mkdir -p "$DATA"

echo "==> emoji-data.txt"
curl -fsSL -o "$DATA/emoji-data.txt" \
  "https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt"

echo "==> emoji-zwj-sequences.txt"
curl -fsSL -o "$DATA/emoji-zwj-sequences.txt" \
  "https://unicode.org/Public/emoji/latest/emoji-zwj-sequences.txt"

echo "==> emoji-sequences.txt"
curl -fsSL -o "$DATA/emoji-sequences.txt" \
  "https://unicode.org/Public/emoji/latest/emoji-sequences.txt"

echo "==> gpt2 tokenizer.json"
curl -fsSL -o "$DATA/gpt2-tokenizer.json" \
  "https://huggingface.co/gpt2/resolve/main/tokenizer.json"

echo "==> CLDR pt-BR (opcional)"
if command -v python3 >/dev/null 2>&1; then
  python3 "$ROOT/scripts/fetch_cldr_pt_br.py" --output "$DATA/cldr_emoji_pt_br.csv" || true
else
  echo "python3 não encontrado; salte cldr_emoji_pt_br.csv"
fi

echo "Pronto. Ficheiros em $DATA"
