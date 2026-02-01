#!/usr/bin/env python3
"""Gera data/cldr_emoji_pt_br.csv a partir das anotações CLDR em português (pt).
Fonte: https://github.com/unicode-org/cldr-json (annotations/pt/annotations.json)
Uso: python3 scripts/fetch_cldr_pt_br.py [--output data/cldr_emoji_pt_br.csv]
"""
import argparse
import csv
import json
import sys
from urllib.request import urlopen

CLDR_PT_URL = "https://raw.githubusercontent.com/unicode-org/cldr-json/main/cldr-json/cldr-annotations-full/annotations/pt/annotations.json"


def main():
    p = argparse.ArgumentParser(description="Fetch CLDR pt emoji annotations → CSV")
    p.add_argument("--output", default="data/cldr_emoji_pt_br.csv", help="Output CSV path")
    args = p.parse_args()

    print("A obter anotações CLDR pt...", file=sys.stderr)
    with urlopen(CLDR_PT_URL) as r:
        data = json.load(r)

    ann = data.get("annotations", {}).get("annotations", {})
    rows = []
    for emoji, meta in ann.items():
        # Preferir "tts" (nome curto para TTS); senão primeiro de "default"
        tts = meta.get("tts")
        default = meta.get("default")
        if tts and len(tts) > 0:
            word = tts[0]
        elif default and len(default) > 0:
            word = default[0]
        else:
            continue
        rows.append((emoji, word))

    out_path = args.output
    with open(out_path, "w", encoding="utf-8", newline="") as f:
        w = csv.writer(f)
        w.writerow(["emoji", "word"])
        w.writerows(rows)

    print(f"Escritos {len(rows)} emojis em {out_path}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
