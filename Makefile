RED    := $(shell tput -Txterm setaf 1)
GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
CYAN   := $(shell tput -Txterm setaf 6)
RESET  := $(shell tput -Txterm sgr0)

.PHONY: all
all: help

## Setup
.PHONY: setup
setup: ## Setup the project
	@echo "${CYAN}🔧 Setting up the project...${RESET}"
	@cargo install systemfd cargo-watch
	@make setup-ui
	@make models
	@echo "${GREEN}✅ Project setup completed!${RESET}"

## UI
.PHONY: ui
ui: ## Build UI components
	@echo "${CYAN}🚀 Building UI components...${RESET}"
	@cd src/ui && make ui
	@echo "${GREEN}✨ UI build completed!${RESET}"

.PHONY: setup-ui
setup-ui: ## Setup UI environment
	@echo "${CYAN}🔧 Setting up UI environment...${RESET}"
	@cd src/ui && make setup-ui
	@echo "${GREEN}✅ UI setup completed!${RESET}"

## Development
.PHONY: serve
serve: ## Start development server
	@echo "${CYAN}🌐 Starting development server...${RESET}"
	@echo "${YELLOW}📡 Browser-Sync listening on http://localhost:8080...${RESET}"
	@trap 'kill $$(jobs -p)' EXIT; \
	npx browser-sync start --logLevel "silent" --proxy "localhost:3000" --port 8080 --files "src/" --no-open --no-ui & \
	systemfd --no-pid -s http::3000 -- cargo watch -x 'run serve'

## Models
.PHONY: models
models: ## Generate models
	@echo "${CYAN}🚀 Generating models...${RESET}"
	@mkdir -p models/potion-base-8M
	@wget https://huggingface.co/minishlab/potion-base-8M/resolve/main/onnx/model.onnx?download=true -O models/potion-base-8M/potion-base-8M.onnx
	@wget https://huggingface.co/minishlab/potion-base-8M/resolve/main/tokenizer.json -O models/potion-base-8M/tokenizer.json
	@echo "${GREEN}✨ Models generated!${RESET}"

## Deployment
.PHONY: deploy
deploy: ## Deploy the application
	@echo "${CYAN}🚀 Deploying ...${RESET}"
	@./docker_build.sh
	@./deploy.sh
	@echo "${GREEN}✨ Deployment completed!${RESET}"

## Linting 
.PHONY: lint
lint: ## Run linters
	@make clippy fmt

.PHONY: lint-fix
lint-fix: ## Run linters and fix issues
	@make fmt-fix clippy-fix

.PHONY: fmt
fmt: ## Format the code
	@echo "${CYAN}📝 Formatting...${RESET}"
	@cargo fmt --all -- --check
	@echo "${GREEN}✨ Formatting completed!${RESET}"

.PHONY: fmt-fix
fmt-fix: ## Format the code and fix issues
	@echo "${CYAN}📝 Formatting...${RESET}"
	@cargo fmt --all
	@echo "${GREEN}✨ Formatting completed!${RESET}"

.PHONY: clippy
clippy: ## Run clippy
	@echo "${CYAN}🔍 Running clippy...${RESET}"
	@cargo clippy --all-targets --all-features
	@echo "${GREEN}✅ Clippy completed!${RESET}"

.PHONY: clippy
clippy-fix: ## Run clippy and fix issues
	@echo "${CYAN}🔍 Running clippy...${RESET}"
	@cargo clippy --all-targets --all-features --fix --allow-dirty
	@echo "${GREEN}✅ Clippy completed!${RESET}"

## Testing
.PHONY: test
test: ## Run tests
	@echo "${CYAN}🧪 Running tests...${RESET}"
	@cargo test
	@echo "${GREEN}✅ Tests completed!${RESET}"

## Scripts
.PHONY: gen_word_dict
gen_word_dict: ## Generate word dictionary
	@echo "${CYAN}🚀 Generating word dictionary...${RESET}"
	@cargo run --bin gen_word_dict
	@echo "${GREEN}✨ Word dictionary generated!${RESET}"

.PHONY: help
help:
	@echo ''
	@echo 'Usage:'
	@echo '  ${YELLOW}make${RESET} ${GREEN}<target>${RESET}'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} { \
		if (/^[a-zA-Z_-]+:.*?##.*$$/) {printf "    ${YELLOW}%-30s${GREEN}%s${RESET}\n", $$1, $$2} \
		else if (/^## .*$$/) {printf "  ${CYAN}%s${RESET}\n", substr($$1,4)} \
		}' $(MAKEFILE_LIST)
