# Changelog

Todas as alterações notáveis ao projeto E2T estão documentadas neste ficheiro. O formato baseia-se em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/).

## [0.1.0] - 2026-02-01

### Adicionado

- Subcomando `fetch-emoji-list`: lê emoji-data.txt (Unicode UTS #51), expande codepoints/intervalos da propriedade Emoji e escreve lista (um emoji por linha).
- Subcomando `build`: carrega tokenizador a partir de tokenizer.json (HuggingFace), tokeniza cada emoji e escreve CSV (emoji, codepoint_hex, tokenizer_id, n_tokens, token_ids, token_strs).
- Subcomando `validate`: lê o CSV e produz relatório com total de linhas, média de tokens por emoji, proporção 1-token, distribuição de n_tokens e estatísticas por tokenizador.
- Parser de emoji-data.txt em `src/emoji_data.rs` (suporte a codepoint único e intervalo; paragem na secção Emoji_Presentation).
- Documentação científica em `docs/pt-br/`: EXPERIMENTO_PESQUISA, DATASET_EMOJI_TOKEN, METRICAS_AVALIACAO, VALIDACAO_DATASET, README.
- Testes unitários (parser, codepoint_hex) e de integração (fetch em fixtures, validate em CSV de amostra).
- Makefile e script `scripts/reproduce.sh` para reprodução.
- CI (GitHub Actions): fmt, clippy, test, build, fetch em fixtures, validate em sample CSV.
- CITATION.cff, LICENSE (MIT), README.md, CHANGELOG.md.

### Limitações conhecidas

- Apenas propriedade Emoji (um codepoint por emoji); emojis compostos (sequências ZWJ, etc.) não são expandidos.
- Tokenizador requer ficheiro local tokenizer.json (sem descarga automática do HuggingFace Hub no Rust).
- Mapeamento emoji → palavra (nome Unicode/CLDR) não incluído no pipeline base.
