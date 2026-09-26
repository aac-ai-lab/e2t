# E2T Emoji para Token/Word — Makefile
#
# Uso: make [alvo]   ou   make help
#
# Alvos principais:
#   release, fetch-emoji-list, build-dictionary (TOKENIZER=...), validate, to-words, reproduce
#
# Variáveis (exemplos):
#   TOKENIZER=data/gpt2-tokenizer.json   (obrigatório em build-dictionary)
#   INPUT="🧒 🍎"   E2T_LANG=pt-br       (to-words: texto e idioma; não usar LANG — conflito com locale)
#   DICTIONARY=data/emoji_dictionary.csv (to-words: ficheiro do dicionário)
#
# Ver README.md secção "Uso com Make" para tabela completa e exemplos.

BIN         = target/release/e2t
EMOJI_DATA  ?= data/emoji-data.txt
EMOJI_LIST  ?= data/emoji_list.txt
TOKENIZER   ?=
TOKENIZER_ID ?= gpt2
CSV_OUT     ?= data/emoji_dictionary.csv
CSV_VALID   ?= data/emoji_dictionary.csv

DICTIONARY ?= data/emoji_dictionary.csv
INPUT       ?=
E2T_LANG    ?=

.PHONY: help build release test fmt fmt-check clippy check clean
.PHONY: fetch-emoji-list build-dictionary validate to-words reproduce install
.PHONY: fetch-data fetch-tokenizer

help:
	@echo "E2T Emoji para Token/Word — alvos disponíveis:"
	@echo ""
	@echo "  make build          Compila (debug)"
	@echo "  make release        Compila em release"
	@echo "  make test           Testes unitários e de integração"
	@echo "  make fmt            Formata o código"
	@echo "  make fmt-check      Verifica formato (CI)"
	@echo "  make clippy         Clippy (avisos = erro)"
	@echo "  make check          fmt-check + clippy + test"
	@echo "  make clean          Remove target/"
	@echo ""
	@echo "  make fetch-data     Descarrega Unicode + tokenizer GPT-2 (+ CLDR pt se houver python3)"
	@echo "  make fetch-tokenizer  Só o tokenizer GPT-2 (atalho)"
	@echo "  make fetch-emoji-list [EMOJI_DATA=data/emoji-data.txt] [EMOJI_LIST=data/emoji_list.txt]"
	@echo "  make build-dictionary   Requer TOKENIZER=/path/to/tokenizer.json [EMOJI_LIST=...] [CSV_OUT=...]"
	@echo "  make validate [CSV_VALID=...]"
	@echo "  make to-words [INPUT=\"🧒 🍎\"] [E2T_LANG=en|pt-br] [DICTIONARY=...]  Converte emojis em palavras"
	@echo "  make reproduce       Requer EMOJI_DATA; opcional TOKENIZER para build completo"
	@echo ""
	@echo "Exemplo:"
	@echo "  make fetch-data && make release && make fetch-emoji-list"
	@echo "  make build-dictionary TOKENIZER=data/gpt2-tokenizer.json CSV_OUT=data/emoji_dictionary.csv"
	@echo "  make validate CSV_VALID=data/emoji_dictionary.csv"
	@echo "  make to-words INPUT=\"🧒 🍎\""
	@echo "  make to-words INPUT=\"🧒 🍎\" E2T_LANG=pt-br"

build:
	cargo build

release:
	cargo build --release

test:
	cargo test --all-targets

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets -- -D warnings

check: fmt-check clippy test
	@echo "check: fmt, clippy e testes OK."

clean:
	cargo clean

fetch-emoji-list: release
	$(BIN) fetch-emoji-list $(EMOJI_DATA) --output $(EMOJI_LIST)

build-dictionary: release
	@if [ -z "$(TOKENIZER)" ]; then echo "Erro: defina TOKENIZER=/path/to/tokenizer.json"; exit 1; fi
	$(BIN) build $(EMOJI_LIST) --tokenizer $(TOKENIZER) --tokenizer-id $(TOKENIZER_ID) --output $(CSV_OUT)

validate: release
	$(BIN) validate $(CSV_VALID)

to-words: release
	@if [ -z "$(INPUT)" ]; then $(BIN) to-words --dictionary $(DICTIONARY); else \
		CMD="$(BIN) to-words \"$(INPUT)\" --dictionary $(DICTIONARY)"; \
		[ -n "$(E2T_LANG)" ] && CMD="$$CMD --lang $(E2T_LANG)"; \
		eval "$$CMD"; \
	fi

reproduce:
	./scripts/reproduce.sh $(EMOJI_DATA) '$(TOKENIZER)' $(CSV_OUT)

install:
	cargo install --path .

fetch-data:
	./scripts/fetch_unicode_and_tokenizer.sh

fetch-tokenizer:
	mkdir -p data
	curl -fsSL -o data/gpt2-tokenizer.json \
	  "https://huggingface.co/gpt2/resolve/main/tokenizer.json"
	@echo "Escrito data/gpt2-tokenizer.json"
