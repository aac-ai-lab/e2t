---
layout: default
title: "Experimento"
---
# Enquadramento do experimento

Este documento enquadra o projeto **E2T Emoji para Token/Word** como **experimento**, para apoio à documentação científica e à redação da tese.

---

## 1. Contexto

- O trabalho desenvolvido neste repositório integra **experimentos** de uma **pesquisa científica**.
- A documentação foi organizada de forma a permitir **reprodutibilidade**, **traçabilidade** do método e **referência** às bases (Unicode UTS #51, tokenizadores, literatura sobre tokenização de emojis).

---

## 2. Objetivo do experimento

- **Mapear todos os emojis existentes** (propriedade Unicode *Emoji*, UTS #51) à sua representação em **token(s)** para um ou mais tokenizadores (ex.: GPT-2, LLaMA), e opcionalmente em **palavra** (nome Unicode/CLDR).
- **Operacionalizar** o mapeamento emoji → token como: para cada codepoint emoji, obter o número de tokens e os identificadores de token gerados pelo tokenizador; documentar a distribuição (1 token vs multi-token).
- **Validar** o dataset com métricas automáticas: distribuição de `n_tokens`, proporção de emojis em 1 token, média de tokens por emoji, estatísticas por tokenizador.
- **Documentar** a fonte dos dados (emoji-data.txt), o método (parser Unicode, tokenizador HuggingFace), as limitações e a reprodutibilidade.

---

## 3. Componentes documentados (para a tese)

| Componente | Documento / local | Uso na redação |
|-----------|-------------------|----------------|
| **Fonte de emojis** | README, `data/emoji-data.txt`, docs/DATASET_EMOJI_TOKEN.md | Unicode UTS #51, emoji-data.txt, primeira secção "Emoji" |
| **Método de mapeamento** | README (“Como funciona”), `src/emoji_data.rs`, `src/cli.rs` | Parser emoji-data, tokenização com tokenizer.json |
| **Pipeline e reprodutibilidade** | README (CLI), Cargo.toml, `scripts/reproduce.sh`, CLI `e2t` | Passos para replicar: fetch-emoji-list → build → validate |
| **Dataset gerado** | docs/DATASET_EMOJI_TOKEN.md, README (Formato do dataset) | Estrutura CSV: emoji, codepoint_hex, tokenizer_id, n_tokens, token_ids, token_strs, word_en, word_pt_br (CLDR ou custom) |
| **Validação e métricas** | docs/VALIDACAO_DATASET.md, docs/METRICAS_AVALIACAO.md, subcomando `e2t validate` | Distribuição de n_tokens, % 1-token, média por tokenizador |
| **Testes e CI** | README (Testes e CI), `tests/`, `tests/fixtures/`, `.github/workflows/ci.yml` | Testes unitários (parser, codepoint_hex); CI (fmt, clippy, test, build) |
| **Limitações** | README, docs | Apenas propriedade Emoji (um codepoint); tokenizador local (ficheiro); word_pt_br depende de data/cldr_emoji_pt_br.csv (CLDR) ou ficheiro custom |
| **Licença e citação** | LICENSE (MIT), CITATION.cff | Licença do software; citação em formato máquina (GitHub/Zenodo) |
| **Histórico** | CHANGELOG.md | Versões e alterações (Keep a Changelog) |

---

## 4. Reprodutibilidade

- **Software:** Rust (Cargo, edição 2021); dependência `tokenizers` (HuggingFace) para tokenização.
- **Dados:** Lista de emojis a partir de [emoji-data.txt](https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt) (Unicode). Tokenizador: ficheiro `tokenizer.json` (ex.: descarregado de HuggingFace, ex. gpt2).
- **Comandos:** `e2t fetch-emoji-list`, `e2t build --tokenizer <path>`, `e2t validate`; ou `./scripts/reproduce.sh`. Execução determinística para o mesmo emoji-data e tokenizador.
- **CI:** Em cada push/PR, o workflow `.github/workflows/ci.yml` executa fmt, clippy, testes unitários e build; opcionalmente fetch-emoji-list e validate em fixtures (sem tokenizador externo na CI).
- **Resultados:** Estatísticas em docs/VALIDACAO_DATASET.md; atualizáveis re-executando o validador.

---

## 5. Limitações (para discussão na tese)

- **Sequências:** a lista completa de compostos (ZWJ, bandeiras, etc.) depende de `emoji-zwj-sequences.txt` / `emoji-sequences.txt` em `data/` (`make fetch-data`). Sem esses ficheiros, só entra a propriedade Emoji de `emoji-data.txt`.
- **Tokenizador:** requer ficheiro local `tokenizer.json`; a descarga do Hub é feita por script/Make (`make fetch-tokenizer`), não dentro do binário Rust.
- **“Word”:** mapeamento emoji → palavra está incluído: **word_en** (nome Unicode em minúsculas; sequências = nomes unidos) e **word_pt_br** (CLDR ou `--words-pt-br`). Subcomando `to-words` com `--lang en` ou `--lang pt-br`.
- **Versão Unicode:** os resultados dependem da versão de emoji-data.txt / sequences utilizada; documentar a versão na validação.

---

## 6. Ética e uso de dados

- Os dados Unicode (emoji-data.txt e ficheiros de sequências) são **públicos** e regidos pelos [termos de uso do Unicode](https://www.unicode.org/terms_of_use.html). Os tokenizadores (ex.: GPT-2) seguem as licenças dos respetivos modelos (HuggingFace, etc.).

---

## 7. Citação do experimento / software

Para citar o experimento ou o pipeline na tese ou em artigos:

> O mapeamento emoji → token foi obtido com o pipeline E2T (Emoji para Token/Word): lista de emojis a partir de Unicode emoji-data.txt e ficheiros de sequências (UTS #51), tokenização com tokenizadores compatíveis HuggingFace (tokenizer.json), dataset CSV com emoji, codepoint_hex, tokenizer_id, n_tokens, token_ids, token_strs, word_en e word_pt_br (nomes em inglês e pt-BR via CLDR ou ficheiro custom); validação com distribuição de n_tokens e estatísticas por tokenizador. Documentação e código: https://github.com/aac-ai-lab/e2t

O repositório inclui **CITATION.cff** para citação automática (GitHub, Zenodo).

---

## 8. Trabalho futuro (sugestões para a tese)

- Suporte a **múltiplos tokenizadores** no mesmo dataset (uma linha por par emoji+tokenizer).
- Comparação entre tokenizadores (proporção 1-token, custo em tokens em modelos de linguagem).
- Integração com **tiktoken** (OpenAI) ou outros backends além de tokenizer.json.
- Anotações CLDR mais ricas para sequências ZWJ (hoje `word_en` compostos usam nomes Unicode unidos quando não há termo CLDR).

---

*Este documento serve de apoio à documentação científica do experimento no âmbito da pesquisa.*
