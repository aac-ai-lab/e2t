---
layout: default
title: "Validação do dataset"
---
# Resultados da validação do dataset emoji → token

Este documento regista o **resultado da validação** obtido ao executar `e2t validate` sobre o CSV gerado. Os valores podem ser reproduzidos com:

```bash
e2t build data/emoji_list.txt --tokenizer <path>/tokenizer.json --tokenizer-id <id> --output data/emoji_dictionary.csv
e2t validate data/emoji_dictionary.csv
```

*(Ficheiro validado: `data/emoji_dictionary.csv`. Lista de emojis: `data/emoji_list.txt` a partir de emoji-data.txt; tokenizador: gpt2.)*

---

## Resumo

| Métrica | Valor |
|--------|--------|
| Total de linhas | 1 438 |
| Média de tokens por emoji | 2,82 |
| Emojis em 1 token | 18 (1,25%) |
| Emojis em >1 token | 1 420 |
| Distribuição n_tokens (1 / 2 / 3) | 18 / 223 / 1 197 |
| Por tokenizer (gpt2) | 1 438 linhas, média 2,82 tokens/emoji |

---

## 1. Estatísticas descritivas

- **Total de emojis (linhas):** **1 438** — gerado por `e2t fetch-emoji-list` a partir de emoji-data.txt (propriedade Emoji, primeira secção).
- **Média de tokens por emoji:** **2,82** — o tokenizador GPT-2 (BPE) representa a maioria dos emojis em 2 ou 3 tokens; apenas 18 emojis (dígitos 0–9, #, *, ©, ®, etc.) ficam em 1 token.
- **Proporção 1-token:** **1,25%** — esperado para GPT-2, treinado sobretudo em texto; a grande maioria dos emojis Unicode é segmentada em vários tokens.

---

## 2. Consistência

- Todas as linhas devem ter `emoji` não vazio, `codepoint_hex` não vazio, `tokenizer_id` não vazio, `n_tokens` ≥ 1 e `token_ids` com exactamente `n_tokens` valores (separados por espaço).

---

## 3. Limitações

- Os resultados são **por tokenizador e por versão de emoji-data**. Alterar o tokenizador ou a lista de emojis altera as métricas.
- Emojis compostos (sequências) não estão incluídos na lista base (um codepoint por emoji).

---

---

## Reprodução

Para reproduzir estes números (com o mesmo emoji-data e tokenizador gpt2):

```bash
e2t fetch-emoji-list data/emoji-data.txt --output data/emoji_list.txt
e2t build data/emoji_list.txt --tokenizer data/gpt2-tokenizer.json --tokenizer-id gpt2 --output data/emoji_dictionary.csv
e2t validate data/emoji_dictionary.csv
```

*Relatório gerado pelo comando `e2t validate`. Para atualizar os números, re-executar o build e o validate.*
