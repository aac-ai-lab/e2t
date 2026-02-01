# Changelog

Todas as alterações notáveis ao projeto E2T estão documentadas neste ficheiro. O formato baseia-se em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/).

## [Unreleased]

### Adicionado

- Colunas **word_en** e **word_pt_br** no CSV gerado pelo `build`: `word_en` = nome Unicode em inglês (minúsculas); `word_pt_br` = termo em pt-BR (só português, sem fallback para inglês).
- Suporte a termos pt-BR via **CLDR**: ficheiro `data/cldr_emoji_pt_br.csv` (anotações oficiais em português); gerar com `python3 scripts/fetch_cldr_pt_br.py`.
- Opção `--words-pt-br <csv>` no `build`: ficheiro custom (emoji, word) para preencher `word_pt_br`; sobrepõe CLDR quando ambos existem.
- Subcomando `to-words`: opção `--lang en` ou `--lang pt-br` para escolher a coluna word_en ou word_pt_br na saída.
- Script `scripts/fetch_cldr_pt_br.py`: descarrega anotações CLDR pt e gera `data/cldr_emoji_pt_br.csv` (formato emoji, word).
- Makefile: alvo `to-words` com variáveis `INPUT`, `E2T_LANG` (não usar `LANG`; conflito com locale), `DICTIONARY`.
- Documentação atualizada: README (uso com Make), docs/pt-br (DATASET_EMOJI_TOKEN, EXPERIMENTO_PESQUISA, METRICAS_AVALIACAO, VALIDACAO_DATASET, README) com word_en, word_pt_br, CLDR e script.

---

## [0.1.0] - 2026-02-01

### Adicionado

- Subcomando `fetch-emoji-list`: lê emoji-data.txt (Unicode UTS #51), expande codepoints/intervalos da propriedade Emoji e escreve lista (um emoji por linha).
- Subcomando `build`: carrega tokenizador a partir de tokenizer.json (HuggingFace), tokeniza cada emoji e escreve CSV (emoji, codepoint_hex, tokenizer_id, n_tokens, token_ids, token_strs, word_en, word_pt_br).
- Subcomando `validate`: lê o CSV e produz relatório com total de linhas, média de tokens por emoji, proporção 1-token, distribuição de n_tokens e estatísticas por tokenizador.
- Subcomando `to-words`: converte frase com emojis em palavras (usa dicionário; colunas word_en ou word_pt_br).
- Parser de emoji-data.txt em `src/emoji_data.rs` (suporte a codepoint único e intervalo; paragem na secção Emoji_Presentation).
- Documentação científica em `docs/pt-br/`: EXPERIMENTO_PESQUISA, DATASET_EMOJI_TOKEN, METRICAS_AVALIACAO, VALIDACAO_DATASET, README.
- Testes unitários (parser, codepoint_hex, emoji_words) e de integração (fetch em fixtures, validate em CSV de amostra).
- Makefile e script `scripts/reproduce.sh` para reprodução.
- CI (GitHub Actions): fmt, clippy, test, build, fetch em fixtures, validate em sample CSV.
- CITATION.cff, LICENSE (MIT), README.md, CHANGELOG.md.

### Limitações conhecidas

- Apenas propriedade Emoji (um codepoint por emoji); emojis compostos (sequências ZWJ, etc.) não são expandidos.
- Tokenizador requer ficheiro local tokenizer.json (sem descarga automática do HuggingFace Hub no Rust).
- word_pt_br depende de data/cldr_emoji_pt_br.csv (gerar com `python3 scripts/fetch_cldr_pt_br.py`) ou de ficheiro custom; emojis sem anotação pt no CLDR ficam com a célula vazia.
