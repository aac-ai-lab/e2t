# Documentação E2T (pt-BR)

Índice da documentação do experimento **E2T Emoji para Token/Word**.

| Documento | Descrição |
|-----------|-----------|
| [EXPERIMENTO_PESQUISA](EXPERIMENTO_PESQUISA.md) | Enquadramento do experimento, objetivo, reprodutibilidade, limitações, citação |
| [DATASET_EMOJI_TOKEN](DATASET_EMOJI_TOKEN.md) | Metodologia e estrutura do dataset emoji → token (CSV: word_en, word_pt_br, CLDR) |
| [METRICAS_AVALIACAO](METRICAS_AVALIACAO.md) | Métricas de avaliação (n_tokens, proporção 1-token, distribuição) |
| [VALIDACAO_DATASET](VALIDACAO_DATASET.md) | Resultados da validação (template e descrição) |

---

O dataset inclui colunas **word_en** (nome Unicode em inglês, minúsculas) e **word_pt_br** (termos em português via [CLDR](https://github.com/unicode-org/cldr-json) ou ficheiro custom; gerar `data/cldr_emoji_pt_br.csv` com `python3 scripts/fetch_cldr_pt_br.py`). O subcomando `to-words` aceita `--lang en` ou `--lang pt-br`.

Ver também o [README principal](../../README.md) do repositório para CLI, comandos, **uso com Make** (alvos, variáveis e exemplos) e reprodução.
