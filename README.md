# E2T Emoji para Token/Word

Projeto Rust para **mapear todos os emojis existentes** (Unicode, propriedade Emoji) à sua representação em **token(s)** para um ou mais tokenizadores, e opcionalmente em **palavra** (nome Unicode/CLDR).

*Este trabalho integra **experimento**; a documentação foi organizada para suportar redação científica e reprodutibilidade. Ver [docs/pt-br/EXPERIMENTO_PESQUISA](docs/pt-br/EXPERIMENTO_PESQUISA.md) para enquadramento do experimento, limitações, citação e uso na tese.*

**Fonte dos emojis:** [Unicode UTS #51](https://unicode.org/reports/tr51), ficheiro [emoji-data.txt](https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt) (primeira secção "Emoji").  
**Tokenização:** Tokenizadores compatíveis com HuggingFace (ficheiro `tokenizer.json`).  
**Saída:** CSV com colunas `emoji`, `codepoint_hex`, `tokenizer_id`, `n_tokens`, `token_ids`, `token_strs`.

**Documentação:** [Índice (pt-BR)](docs/pt-br/README.md) · [Experimento](docs/pt-br/EXPERIMENTO_PESQUISA.md) · [Dataset](docs/pt-br/DATASET_EMOJI_TOKEN.md) · [Métricas](docs/pt-br/METRICAS_AVALIACAO.md) · [Validação](docs/pt-br/VALIDACAO_DATASET.md). **Citação:** [CITATION.cff](CITATION.cff). **Histórico:** [CHANGELOG](CHANGELOG.md).

## Como funciona (replicabilidade)

1. **Lista de emojis:** O subcomando `fetch-emoji-list` lê `emoji-data.txt`, interpreta linhas `XXXX ; Emoji` e `XXXX..YYYY ; Emoji`, expande intervalos e escreve um emoji por linha (um codepoint por linha). A lista segue a propriedade **Emoji** (UTS #51); emojis compostos (sequências ZWJ, etc.) não são expandidos nesta versão.
2. **Tokenização:** O subcomando `build` carrega um tokenizador a partir de `tokenizer.json` (formato HuggingFace), codifica cada emoji como string e regista `n_tokens`, `token_ids` e `token_strs`.
3. **Validação:** O subcomando `validate` lê o CSV e produz estatísticas: total de linhas, média de tokens por emoji, proporção de emojis em 1 token, distribuição de `n_tokens`, estatísticas por tokenizador.

O processo é **determinístico** para a mesma versão de emoji-data e do mesmo ficheiro tokenizador.

## Requisitos

- [Rust](https://www.rust-lang.org/) (edição 2021; `rustup` recomendado)
- Ficheiro **emoji-data.txt** (descarregar de [Unicode](https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt)) em `data/` ou indicar caminho
- Ficheiro **tokenizer.json** (ex.: [GPT-2 no HuggingFace](https://huggingface.co/gpt2)) para construir o dataset completo

## Estrutura

```
e2t/
├── .github/workflows/ci.yml    # CI: fmt, clippy, test, build, fetch (fixtures), validate (sample)
├── Cargo.toml
├── data/
│   ├── emoji-data.txt          # (opcional) descarregar do Unicode
│   ├── emoji_list.txt          # gerado por fetch-emoji-list
│   ├── emoji_token_dataset.csv # gerado por build
│   └── emoji_token_dataset_sample.csv  # amostra para testes
├── docs/pt-br/                 # documentação científica
├── scripts/reproduce.sh
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── cli.rs
│   ├── emoji_data.rs           # parser emoji-data.txt
│   └── metrics.rs              # validação e estatísticas
├── tests/
│   ├── integration_test.rs
│   └── fixtures/emoji-data-sample.txt
├── Makefile
├── README.md
├── CITATION.cff
├── LICENSE
└── CHANGELOG.md
```

## CLI (binário único)

```bash
cargo build --release   # gera target/release/e2t
cargo run --release -- <subcomando> [opções]
```

**Subcomandos:**

| Subcomando | Descrição |
|------------|-----------|
| `fetch-emoji-list [input] [--output]` | Lê emoji-data.txt e escreve lista de emojis (um por linha) |
| `build [emoji_list] --tokenizer <path> [--tokenizer-id] [--output] [--from-emoji-data]` | Tokeniza cada emoji e escreve CSV |
| `validate [csv]` | Valida o dataset: estatísticas e distribuição de n_tokens |

### 1. Obter lista de emojis

```bash
# Descarregar emoji-data.txt (uma vez):
# wget -O data/emoji-data.txt https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt

cargo run --release -- fetch-emoji-list
# ou:
cargo run --release -- fetch-emoji-list data/emoji-data.txt --output data/emoji_list.txt
```

### 2. Construir o dataset (requer tokenizer.json)

```bash
# Exemplo: descarregar tokenizer GPT-2
# wget -O data/gpt2-tokenizer.json https://huggingface.co/gpt2/raw/main/tokenizer.json

cargo run --release -- build data/emoji_list.txt \
  --tokenizer data/gpt2-tokenizer.json \
  --tokenizer-id gpt2 \
  --output data/emoji_token_dataset.csv
```

Ou usar emoji-data.txt diretamente:

```bash
cargo run --release -- build data/emoji-data.txt \
  --from-emoji-data \
  --tokenizer data/gpt2-tokenizer.json \
  --tokenizer-id gpt2 \
  --output data/emoji_token_dataset.csv
```

### 3. Validar o dataset

```bash
cargo run --release -- validate data/emoji_token_dataset.csv
```

## Formato do dataset (CSV)

| Coluna          | Descrição |
|-----------------|-----------|
| `emoji`         | Caractere emoji (um codepoint) |
| `codepoint_hex` | Codepoint em hexadecimal (ex.: 1F600) |
| `tokenizer_id`  | Identificador do tokenizador (ex.: gpt2) |
| `n_tokens`      | Número de tokens gerados |
| `token_ids`     | IDs dos tokens (separados por espaço) |
| `token_strs`    | Strings dos tokens (separadas por espaço) |

## Reprodução (um comando)

```bash
./scripts/reproduce.sh [emoji_data_path] [tokenizer_path] [csv_out]
# Sem tokenizer: só fetch (se emoji-data existir) e validate do CSV de amostra.
# Com tokenizer: fetch → build → validate.
```

**Makefile:** `make help`, `make release`, `make test`, `make fetch-emoji-list`, `make build-dataset TOKENIZER=...`, `make validate`, `make reproduce`.

## Testes e CI

- **Testes unitários:** `cargo test` (parser emoji-data, codepoint_hex).
- **Testes de integração:** `tests/integration_test.rs` — fetch em fixtures, validate no CSV de amostra.
- **CI (GitHub Actions):** em cada push/PR em `main`/`master`, executa fmt, clippy, testes, build release, fetch em fixtures e validate no CSV de amostra.

## Limitações

- Apenas a **propriedade Emoji** (primeira secção de emoji-data.txt); emojis compostos (sequências) não são expandidos.
- Tokenizador requer **ficheiro local** `tokenizer.json` (sem descarga automática do Hub no Rust).
- Mapeamento emoji → **palavra** (nome Unicode/CLDR) não está incluído no pipeline base.

## Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE).
