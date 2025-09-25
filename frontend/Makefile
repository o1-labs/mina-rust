# Frontend Makefile

.PHONY: help
help: ## Display this help message
	@echo "Frontend Makefile - Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; \
		{printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build the frontend
	ng build

.PHONY: build-dev
build-dev: ## Build the frontend for development
	ng build --configuration development

.PHONY: build-prod
build-prod: ## Build the frontend for production
	ng build --configuration production && $(MAKE) sentry-sourcemaps

.PHONY: check-prettify
check-prettify: ## Check if files are formatted with Prettier
	npx prettier --check 'src/**/*.{ts,js,html,scss,css,json}'

.PHONY: clean
clean: ## Clean build artifacts and node_modules
	rm -rf dist node_modules

.PHONY: copy-env
copy-env: ## Copy webnode.js to env.js
	cp dist/frontend/browser/assets/environments/webnode.js \
		dist/frontend/browser/assets/environments/env.js

.PHONY: deploy
deploy: ## Deploy the application
	$(MAKE) prebuild && $(MAKE) build-prod && $(MAKE) copy-env && \
		firebase deploy

.PHONY: deploy-leaderboard
deploy-leaderboard: ## Deploy the leaderboard application
	$(MAKE) prebuild && $(MAKE) build-prod && \
		cp dist/frontend/browser/assets/environments/leaderboard.js \
		dist/frontend/browser/assets/environments/env.js && \
		firebase deploy

.PHONY: docker
docker: ## Build and push Docker image
	$(MAKE) build-prod && \
		docker buildx build --platform linux/amd64 -t openmina/frontend:latest . && \
		docker push openmina/frontend:latest

.PHONY: install
install: ## Install npm dependencies
	npm install

.PHONY: install-deps
install-deps: ## Install npm dependencies (alias)
	npm install

.PHONY: prebuild
prebuild: ## Run prebuild script to update version
	node scripts/update-frontend-version.js

.PHONY: prettify
prettify: ## Format HTML, SCSS, TypeScript, and JavaScript files with Prettier
	npx prettier --write 'src/**/*.{ts,js,html,scss,css,json}'

.PHONY: rebuild
rebuild: clean install build ## Clean, reinstall dependencies, and rebuild

.PHONY: sentry-sourcemaps
sentry-sourcemaps: ## Upload sourcemaps to Sentry
	sentry-cli sourcemaps inject --org openmina-uv --project openmina \
		./dist/frontend/browser && \
		sentry-cli sourcemaps upload --org openmina-uv --project openmina \
		./dist/frontend/browser

.PHONY: start
start: ## Start the development server
	npm install && ng serve --configuration local --open

.PHONY: start-bundle
start-bundle: ## Serve the built bundle
	serve dist/frontend/browser -s -l 4200

.PHONY: start-dev
start-dev: ## Start the development server (dev configuration)
	ng serve --configuration development

.PHONY: start-dev-mobile
start-dev-mobile: ## Start the development server for mobile (accessible on network)
	ng serve --configuration development --host 0.0.0.0

.PHONY: start-fuzzing
start-fuzzing: ## Start the fuzzing build and serve
	$(MAKE) install-deps && ng build --configuration fuzzing && \
		$(MAKE) start-bundle

.PHONY: start-webnode
start-webnode: ## Start the webnode development server
	npm install && ng serve --configuration webnodelocal --open

.PHONY: test
test: ## Run Cypress tests
	npx cypress open --config baseUrl=http://localhost:4200

.PHONY: test-headless
test-headless: ## Run Cypress tests in headless mode
	npx cypress run --headless --config baseUrl=http://localhost:4200