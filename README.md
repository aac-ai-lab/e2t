# E2T Emoji para Token/Word

Projeto Rust para **gerar automaticamente o dicionário completo** emoji → token/word: para cada emoji (Unicode, propriedade Emoji) regista a tokenização (tokenizador) e as **palavras** em inglês (`word_en`, nome Unicode em minúsculas) e em pt-BR (`word_pt_br`, via CLDR ou ficheiro custom). O ficheiro gerado **é** o dicionário — não há dataset separado.

*Este trabalho integra **experimento**; a documentação foi organizada para suportar redação científica e reprodutibilidade. Ver [docs/pt-br/EXPERIMENTO_PESQUISA](docs/pt-br/EXPERIMENTO_PESQUISA.md) para enquadramento do experimento, limitações, citação e uso na tese.*

**Fonte dos emojis:** [Unicode UTS #51](https://unicode.org/reports/tr51), ficheiro [emoji-data.txt](https://unicode.org/Public/UCD/latest/ucd/emoji/emoji-data.txt) (primeira secção "Emoji").  
**Tokenização:** Tokenizadores compatíveis com HuggingFace (ficheiro `tokenizer.json`).  
**Palavra (word):** Nome Unicode em minúsculas em **inglês** (`word_en`) e opcionalmente em **pt-BR** (`word_pt_br`) via [CLDR](https://github.com/unicode-org/cldr-json) ou ficheiro custom.  
**Saída:** Um único CSV que serve de **dicionário** — colunas `emoji`, `codepoint_hex`, `tokenizer_id`, `n_tokens`, `token_ids`, `token_strs`, `word_en`, `word_pt_br`.

**Documentação:** [Índice (pt-BR)](docs/pt-br/README.md) · [Experimento](docs/pt-br/EXPERIMENTO_PESQUISA.md) · [Dataset](docs/pt-br/DATASET_EMOJI_TOKEN.md) · [Métricas](docs/pt-br/METRICAS_AVALIACAO.md) · [Validação](docs/pt-br/VALIDACAO_DATASET.md). **Citação:** [CITATION.cff](CITATION.cff). **Histórico:** [CHANGELOG](CHANGELOG.md).

## Como funciona (replicabilidade)

1. **Lista de emojis:** O subcomando `fetch-emoji-list` lê `emoji-data.txt`, interpreta linhas `XXXX ; Emoji` e `XXXX..YYYY ; Emoji`, expande intervalos e escreve um emoji por linha (um codepoint por linha). A lista segue a propriedade **Emoji** (UTS #51); emojis compostos (sequências ZWJ, etc.) não são expandidos nesta versão.
2. **Construção do dicionário:** O subcomando `build` gera o **dicionário completo** emoji → token/word: para cada emoji regista `n_tokens`, `token_ids`, `token_strs`, **`word_en`** (nome Unicode em minúsculas) e **`word_pt_br`** (termos em português: `data/cldr_emoji_pt_br.csv` do CLDR ou `--words-pt-br` para custom).
3. **Validação:** O subcomando `validate` lê o dicionário (CSV) e produz estatísticas: total de linhas, média de tokens por emoji, proporção de emojis em 1 token, distribuição de `n_tokens`, estatísticas por tokenizador.
4. **Emoji → palavra (to-words):** Usa o dicionário gerado: dada uma frase com emojis (ex.: `[🧒, 🍎]`), devolve a sequência de palavras. Use `--lang en` (padrão) ou `--lang pt-br` para escolher `word_en` ou `word_pt_br`.

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
│   ├── emoji_dictionary.csv # dicionário completo (build): emoji → token + word (nome Unicode)
│   ├── emoji_dictionary_sample.csv  # amostra para testes
│   ├── cldr_emoji_pt_br.csv    # termos pt-BR do CLDR (gerar: python3 scripts/fetch_cldr_pt_br.py)
│   ├── emoji_words_pt_br.csv   # opcional: termos custom pt-BR para --words-pt-br
│   └── emoji_words.csv         # opcional: mapeamentos custom (emoji, word)
├── docs/pt-br/                 # documentação científica
├── scripts/reproduce.sh
├── scripts/fetch_cldr_pt_br.py  # gera data/cldr_emoji_pt_br.csv a partir do CLDR
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── cli.rs
│   ├── emoji_data.rs           # parser emoji-data.txt
│   ├── emoji_words.rs          # carregar dicionário emoji→palavra (word_en, word_pt_br)
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
| `build [emoji_list] --tokenizer <path> [--words-pt-br <csv>] [--output] [--from-emoji-data]` | Tokeniza cada emoji e escreve CSV (word_en + word_pt_br) |
| `validate [csv]` | Valida o dataset: estatísticas e distribuição de n_tokens |
| `to-words [input] [--dictionary] [--lang en\|pt-br] [--input-file]` | Converte frase com emojis em palavras (usa word_en ou word_pt_br) |

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
  --output data/emoji_dictionary.csv

# Com termos pt-BR: gerar data/cldr_emoji_pt_br.csv (uma vez) ou passar --words-pt-br
# python3 scripts/fetch_cldr_pt_br.py --output data/cldr_emoji_pt_br.csv
cargo run --release -- build data/emoji_list.txt \
  --tokenizer data/gpt2-tokenizer.json \
  --words-pt-br data/emoji_words_pt_br.csv \
  --output data/emoji_dictionary.csv
```

Ou usar emoji-data.txt diretamente:

```bash
cargo run --release -- build data/emoji-data.txt \
  --from-emoji-data \
  --tokenizer data/gpt2-tokenizer.json \
  --tokenizer-id gpt2 \
  --output data/emoji_dictionary.csv
```

### 3. Validar o dataset

```bash
cargo run --release -- validate data/emoji_dictionary.csv
```

### 4. Emoji → palavra (to-words)

Converte uma frase com emojis na sequência de **palavras** do dicionário (por defeito: `data/emoji_dictionary.csv`). O dicionário é o ficheiro gerado por `build` — colunas **`word_en`** (inglês, nome Unicode em minúsculas) e **`word_pt_br`** (pt-BR, via CLDR ou `--words-pt-br`).

**Entrada:** lista de emojis como `[🧒, 🍎]` ou `🧒 🍎` (separados por vírgula ou espaço).  
**Saída:** palavras separadas por espaço. Use `--lang en` (padrão) ou `--lang pt-br` para escolher a coluna.

```bash
# Inglês (padrão): word_en
e2t to-words "[🧒, 🍎]"
# → child red apple

# Português: word_pt_br (se o dicionário tiver essa coluna preenchida)
e2t to-words "[🧒, 🍎]" --lang pt-br
# → criança maçã

e2t to-words "🧒 🍎"

# Dicionário alternativo (ex.: emoji_words.csv)
e2t to-words "[🧒, 🍎]" --dictionary data/emoji_words.csv

# Ler entrada de ficheiro
e2t to-words "" --input-file frase.txt
```

O **dicionário** é o CSV gerado por `build`: colunas `word_en` (nome Unicode em minúsculas) e `word_pt_br` (opcional; preencher com `data/cldr_emoji_pt_br.csv` — gerar com `python3 scripts/fetch_cldr_pt_br.py` — ou `--words-pt-br`).

## Formato do dicionário (CSV)

| Coluna          | Descrição |
|-----------------|-----------|
| `emoji`         | Caractere emoji (um codepoint) |
| `codepoint_hex` | Codepoint em hexadecimal (ex.: 1F600) |
| `tokenizer_id`  | Identificador do tokenizador (ex.: gpt2) |
| `n_tokens`      | Número de tokens gerados |
| `token_ids`     | IDs dos tokens (separados por espaço) |
| `token_strs`    | Strings dos tokens (separadas por espaço) |
| `word_en`       | Nome Unicode em inglês, minúsculas (ex.: grinning face), gerado no build |
| `word_pt_br`    | Termo em pt-BR: `--words-pt-br` (custom) ou `data/cldr_emoji_pt_br.csv` (CLDR pt; gerar com `python3 scripts/fetch_cldr_pt_br.py`) — só português, sem fallback para inglês |

## Reprodução (um comando)

```bash
./scripts/reproduce.sh [emoji_data_path] [tokenizer_path] [csv_out]
# Sem tokenizer: só fetch (se emoji-data existir) e validate do CSV de amostra.
# Com tokenizer: fetch → build → validate.
```

---

## Uso com Make

O Makefile expõe os subcomandos do E2T como alvos. Lista completa: `make help`.

### Alvos principais

| Alvo | Descrição |
|------|-----------|
| `make release` | Compila o binário em modo release (`target/release/e2t`) |
| `make fetch-emoji-list` | Gera a lista de emojis (usa `EMOJI_DATA`, escreve em `EMOJI_LIST`) |
| `make build-dictionary` | Constrói o CSV do dicionário — **requer** `TOKENIZER=...` |
| `make validate` | Valida o CSV e mostra estatísticas |
| `make to-words` | Converte emojis em palavras (usa `INPUT`, `E2T_LANG`, `DICTIONARY`) |
| `make reproduce` | Executa fetch → build → validate (script `reproduce.sh`) |

### Variáveis (opcionais)

| Variável | Alvo(s) | Valor por defeito | Descrição |
|----------|---------|-------------------|-----------|
| `EMOJI_DATA` | fetch-emoji-list, reproduce | `data/emoji-data.txt` | Ficheiro emoji-data.txt |
| `EMOJI_LIST` | fetch-emoji-list, build-dictionary | `data/emoji_list.txt` | Lista de emojis (um por linha) |
| `TOKENIZER` | build-dictionary, reproduce | — | Caminho para `tokenizer.json` (obrigatório em build-dictionary) |
| `TOKENIZER_ID` | build-dictionary | `gpt2` | Identificador do tokenizador no CSV |
| `CSV_OUT` | build-dictionary, reproduce | `data/emoji_dictionary.csv` | CSV de saída do build |
| `CSV_VALID` | validate | `data/emoji_dictionary.csv` | CSV a validar |
| `DICTIONARY` | to-words | `data/emoji_dictionary.csv` | CSV do dicionário para to-words |
| `INPUT` | to-words | (vazio) | Texto com emojis a converter; se vazio, lê de stdin |
| `E2T_LANG` | to-words | (vazio) | `en` ou `pt-br` — coluna word_en ou word_pt_br (não usar `LANG`; conflito com locale do sistema) |

### Exemplos com make

```bash
# Ver todos os alvos
make help

# Compilar e gerar lista de emojis
make release && make fetch-emoji-list

# Construir dicionário (obrigatório indicar TOKENIZER)
make build-dictionary TOKENIZER=data/gpt2-tokenizer.json

# Com termos pt-BR (CLDR em data/cldr_emoji_pt_br.csv ou custom)
make build-dictionary TOKENIZER=data/gpt2-tokenizer.json \
  CSV_OUT=data/emoji_dictionary.csv

# Validar o CSV gerado
make validate
# ou com outro ficheiro:
make validate CSV_VALID=data/emoji_dictionary.csv

# Emojis → palavras (inglês, padrão)
make to-words INPUT="🧒 🍎"
# → child red apple

# Emojis → palavras em pt-BR
make to-words INPUT="🧒 🍎" E2T_LANG=pt-br
# → criança maçã

# to-words com dicionário alternativo
make to-words INPUT="😀 👍" DICTIONARY=data/emoji_dictionary.csv E2T_LANG=pt-br

# Reprodução completa (fetch + build + validate)
make reproduce EMOJI_DATA=data/emoji-data.txt TOKENIZER=data/gpt2-tokenizer.json
```

**Nota:** Para `make to-words` é necessário passar o texto na variável **`INPUT=`** (por exemplo `make to-words INPUT="🧒 🍎"`). O comando `make to-words "🧒 🍎"` não funciona: o Make interpreta `"🧒 🍎"` como outro alvo.

## Testes e CI

- **Testes unitários:** `cargo test` (parser emoji-data, codepoint_hex).
- **Testes de integração:** `tests/integration_test.rs` — fetch em fixtures, validate no CSV de amostra.
- **CI (GitHub Actions):** em cada push/PR em `gh-pages`, `main` ou `master`, executa fmt, clippy, testes, build release, fetch em fixtures e validate no CSV de amostra.

## Limitações

- Apenas a **propriedade Emoji** (primeira secção de emoji-data.txt); emojis compostos (sequências) não são expandidos.
- Tokenizador requer **ficheiro local** `tokenizer.json` (sem descarga automática do Hub no Rust).
- `word_pt_br` depende de `data/cldr_emoji_pt_br.csv` (gerar com `python3 scripts/fetch_cldr_pt_br.py`) ou de ficheiro custom; emojis sem anotação pt no CLDR ficam com a célula vazia.

## Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE).
