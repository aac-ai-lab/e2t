---
layout: default
title: "Validação do dataset"
---
# Resultados da validação do dataset emoji → token

Este documento regista o **resultado da validação** obtido ao executar `e2t validate` sobre o CSV gerado. Os valores podem ser reproduzidos com:

```bash
e2t build --emoji-list data/emoji_list.txt --tokenizer <path>/tokenizer.json --tokenizer-id <id> --output data/emoji_token_dataset.csv
e2t validate data/emoji_token_dataset.csv
```

*(A lista de emojis depende de `data/emoji_list.txt` ou de `data/emoji-data.txt`; o tokenizador depende do ficheiro indicado em `--tokenizer`.)*

---

## Resumo (exemplo)

| Métrica | Descrição |
|--------|------------|
| Total de linhas | Número de pares (emoji, tokenização) no CSV |
| Média de tokens por emoji | Soma de n_tokens / total de linhas |
| Emojis em 1 token | Contagem e percentagem de linhas com n_tokens == 1 |
| Emojis em >1 token | Restantes linhas |
| Distribuição de n_tokens | Histograma (1, 2, 3, … tokens) |
| Por tokenizer | Se houver vários tokenizer_id, médias e contagens por id |

*(Os valores numéricos concretos dependem da versão de emoji-data, do tokenizador e do número de emojis processados. Preencher após uma execução real ou manter como template.)*

---

## 1. Estatísticas descritivas

- **Total de emojis (linhas):** Conforme gerado por `e2t fetch-emoji-list` a partir de emoji-data.txt (ex.: ~1424 para a secção “Emoji” do Unicode 15.0).
- **Média de tokens por emoji:** Típico para tokenizadores BPE (ex.: GPT-2) que não têm vocabulário específico para emojis: muitos emojis em 2–4 tokens; média > 1.
- **Proporção 1-token:** Em tokenizadores treinados sobretudo em texto ASCII/UTF-8 comum, a proporção de emojis em 1 token pode ser baixa; tokenizadores com mais cobertura de Unicode podem ter percentagem mais alta.

---

## 2. Consistência

- Todas as linhas devem ter `emoji` não vazio, `codepoint_hex` não vazio, `tokenizer_id` não vazio, `n_tokens` ≥ 1 e `token_ids` com exactamente `n_tokens` valores (separados por espaço).

---

## 3. Limitações

- Os resultados são **por tokenizador e por versão de emoji-data**. Alterar o tokenizador ou a lista de emojis altera as métricas.
- Emojis compostos (sequências) não estão incluídos na lista base (um codepoint por emoji).

---

*Relatório gerado pelo comando `e2t validate`. Para atualizar os números, re-executar o build e o validate.*
