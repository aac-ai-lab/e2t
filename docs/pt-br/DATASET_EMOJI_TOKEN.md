---
layout: default
title: "Dataset emoji → token"
---
# Dataset emoji → token

Este documento descreve a metodologia e a estrutura do **dataset emoji → token** gerado pelo pipeline E2T: para cada emoji (Unicode, propriedade Emoji), regista a sua tokenização (número de tokens, IDs e strings de tokens) para um tokenizador dado, e as colunas **word_en** (nome Unicode em inglês, minúsculas) e **word_pt_br** (termos em português via CLDR ou ficheiro custom).

---

## 1. Nome e natureza

- **Nome:** Dataset emoji → token (E2T).
- **Tipo:** dataset **sintético** — obtido por processamento automático: lista de emojis (emoji-data.txt) + tokenização (tokenizer.json).
- **Fonte dos emojis:** [Unicode UTS #51](https://unicode.org/reports/tr51), ficheiro [emoji-data.txt](https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt), primeira secção com propriedade **Emoji** (codepoints e intervalos).

---

## 2. Metodologia

1. **Lista de emojis:** Obter emoji-data.txt (descarregar ou usar cópia em `data/emoji-data.txt`). O subcomando `e2t fetch-emoji-list` lê emoji-data.txt, interpreta linhas `XXXX ; Emoji` e `XXXX..YYYY ; Emoji`, expande intervalos e escreve um emoji por linha em `data/emoji_list.txt`.
2. **Tokenização:** Para cada emoji (um `char` por linha), o subcomando `e2t build` carrega um tokenizador a partir de `tokenizer.json` (formato HuggingFace), codifica o emoji como string e regista: `n_tokens`, lista de `token_ids` e `token_strs`.
3. **Saída:** CSV com colunas descritas abaixo. Uma linha por emoji (e, em extensões futuras, uma linha por par emoji + tokenizador).

O processo é **determinístico** para a mesma versão de emoji-data e do mesmo ficheiro tokenizador.

---

## 3. Estrutura do CSV

| Coluna          | Descrição |
|-----------------|-----------|
| `emoji`         | Caractere emoji (um codepoint) |
| `codepoint_hex` | Codepoint em hexadecimal (ex.: 1F600, 0023) |
| `tokenizer_id`  | Identificador do tokenizador (ex.: gpt2, LLaMA-2) |
| `n_tokens`      | Número de tokens gerados para este emoji |
| `token_ids`     | IDs dos tokens separados por espaço |
| `token_strs`    | Strings dos tokens separadas por espaço (representação no vocabulário) |
| `word_en`       | Nome Unicode em inglês, minúsculas (ex.: grinning face), gerado no build |
| `word_pt_br`    | Termo em pt-BR: preenchido a partir de `data/cldr_emoji_pt_br.csv` (CLDR pt; gerar com `python3 scripts/fetch_cldr_pt_br.py`) ou de `--words-pt-br` (ficheiro custom). Só português; sem fallback para inglês. |

Encoding: UTF-8. Separador CSV: vírgula. Aspas quando necessário.

---

## 4. Geração

```bash
# 1. Obter lista de emojis (requer emoji-data.txt em data/)
e2t fetch-emoji-list data/emoji-data.txt --output data/emoji_list.txt

# 2. (Opcional) Gerar termos pt-BR do CLDR para preencher word_pt_br
python3 scripts/fetch_cldr_pt_br.py --output data/cldr_emoji_pt_br.csv

# 3. Descarregar tokenizer (ex.: GPT-2) e construir dataset
#    Exemplo: wget -O data/gpt2-tokenizer.json https://huggingface.co/gpt2/raw/main/tokenizer.json
e2t build data/emoji_list.txt --tokenizer data/gpt2-tokenizer.json --tokenizer-id gpt2 --output data/emoji_dictionary.csv
# Com termos custom pt-BR: --words-pt-br data/emoji_words_pt_br.csv

# 4. Validar
e2t validate data/emoji_dictionary.csv
```

Alternativa: usar emoji-data.txt diretamente no build com `--from-emoji-data`:

```bash
e2t build data/emoji-data.txt --from-emoji-data --tokenizer data/gpt2-tokenizer.json --tokenizer-id gpt2 --output data/emoji_dictionary.csv
```

---

## 5. Uso na pesquisa

- Análise da **granularidade** com que emojis são representados por tokenizadores (1 token vs vários).
- Comparação entre **tokenizadores** (ex.: GPT-2 vs LLaMA) em termos de número de tokens por emoji.
- Base para estudos de **custos** (tokens) em modelos de linguagem quando o texto contém emojis.
- Mapeamento emoji → **palavra**: colunas **word_en** (nome Unicode em inglês) e **word_pt_br** (termos em português via CLDR ou ficheiro custom); subcomando `to-words` com `--lang en` ou `--lang pt-br`.

---

*Ver também: [EXPERIMENTO_PESQUISA](EXPERIMENTO_PESQUISA.md), [METRICAS_AVALIACAO](METRICAS_AVALIACAO.md), [VALIDACAO_DATASET](VALIDACAO_DATASET.md).*
