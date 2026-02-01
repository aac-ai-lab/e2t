---
layout: default
title: "Métricas de avaliação"
---
# Métricas de avaliação (E2T)

O projeto inclui **avaliação automática** do dataset emoji → token com as seguintes métricas.

---

## 1. Número de tokens por emoji (n_tokens)

- **O que mede:** Para cada emoji, o número de tokens que o tokenizador produz. Permite saber quantos emojis são representados por 1 token vs 2 ou mais.
- **Valor:** Inteiro ≥ 1.
- **Implementação:** Calculado no `e2t build` e guardado na coluna `n_tokens`; o validador agrega estatísticas.

---

## 2. Proporção 1-token (single-token)

- **O que mede:** Percentagem de emojis que são tokenizados como **um único token**.
- **Valor:** 0–100 %.
- **Implementação:** Rust em `src/metrics.rs` (validador): conta linhas com `n_tokens == 1` e divide pelo total.

---

## 3. Média de tokens por emoji

- **O que mede:** Média de `n_tokens` sobre todos os emojis do dataset. Indica o “custo” médio em tokens por emoji para aquele tokenizador.
- **Valor:** ≥ 1.0 (float).
- **Implementação:** Rust no validador (`ValidationReport::mean_tokens_per_emoji`).

---

## 4. Distribuição de n_tokens

- **O que mede:** Contagem de emojis por valor de `n_tokens` (1, 2, 3, …). Mostra a forma da distribuição (quantos emojis em 1 token, 2 tokens, etc.).
- **Valor:** Histograma (n_tokens → contagem).
- **Implementação:** Rust no validador (`ValidationReport::n_tokens_distribution`).

---

## 5. Estatísticas por tokenizador

- **O que mede:** Quando o dataset inclui várias colunas ou ficheiros por tokenizador (ou múltiplas execuções com `tokenizer_id` diferente), o validador agrupa por `tokenizer_id`: número de linhas e média de tokens por emoji por tokenizador.
- **Implementação:** Rust no validador (`ValidationReport::by_tokenizer`).

---

## Como obter as métricas

```bash
e2t validate data/emoji_dictionary.csv
```

O relatório impresso inclui: total de linhas, média de tokens por emoji, contagem e percentagem de emojis em 1 token, distribuição de n_tokens (amostra) e, se aplicável, estatísticas por tokenizador.

---

*Convenção: cada linha do CSV = um emoji + um tokenizador; as métricas são calculadas sobre o conjunto de todas as linhas (ou por tokenizer_id).*
