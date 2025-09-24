# Frontend Makefile

.PHONY: help
help: ## Display this help message
	@echo "Frontend Makefile - Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: install
install: ## Install npm dependencies
	npm install

.PHONY: prettify
prettify: ## Format HTML, SCSS, TypeScript, and JavaScript files with Prettier
	npx prettier --write 'src/**/*.{ts,js,html,scss,css,json}'

.PHONY: check-prettify
check-prettify: ## Check if files are formatted with Prettier
	npx prettier --check 'src/**/*.{ts,js,html,scss,css,json}'

.PHONY: start
start: ## Start the development server
	npm start

.PHONY: build
build: ## Build the frontend for production
	npm run build:prod

.PHONY: test
test: ## Run Cypress tests
	npm run tests

.PHONY: test-headless
test-headless: ## Run Cypress tests in headless mode
	npm run tests:headless

.PHONY: clean
clean: ## Clean build artifacts and node_modules
	rm -rf dist node_modules

.PHONY: rebuild
rebuild: clean install build ## Clean, reinstall dependencies, and rebuild
