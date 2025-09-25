# Frontend Makefile

.PHONY: help
help: ## Display this help message
	@echo "Frontend Makefile - Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; \
		{printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build the frontend
	npx ng build

.PHONY: build-development
build-development: ## Build the frontend with development configuration
	npx ng build --configuration development

.PHONY: build-fuzzing
build-fuzzing: ## Build the frontend with fuzzing configuration
	npx ng build --configuration fuzzing

.PHONY: build-local
build-local: ## Build the frontend with local configuration
	npx ng build --configuration local

.PHONY: build-prod
build-prod: ## Build the frontend for production
	npx ng build --configuration production

.PHONY: build-prod-sentry
build-prod-sentry: ## Build the frontend for production with Sentry sourcemaps
	npx ng build --configuration production
	$(MAKE) sentry-sourcemaps

.PHONY: build-producer
build-producer: ## Build the frontend with producer configuration
	npx ng build --configuration producer

.PHONY: build-production
build-production: ## Build the frontend with production configuration
	npx ng build --configuration production

.PHONY: build-webnodelocal
build-webnodelocal: ## Build the frontend with webnodelocal configuration
	npx ng build --configuration webnodelocal

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
	$(MAKE) prebuild
	$(MAKE) build-prod-sentry
	$(MAKE) copy-env
	firebase deploy

.PHONY: deploy-leaderboard
deploy-leaderboard: ## Deploy the leaderboard application
	$(MAKE) prebuild
	$(MAKE) build-prod
	cp dist/frontend/browser/assets/environments/leaderboard.js \
		dist/frontend/browser/assets/environments/env.js
	firebase deploy

.PHONY: deploy-leaderboard-sentry
deploy-leaderboard-sentry: ## Deploy the leaderboard application with Sentry sourcemaps
	$(MAKE) prebuild
	$(MAKE) build-prod-sentry
	cp dist/frontend/browser/assets/environments/leaderboard.js \
		dist/frontend/browser/assets/environments/env.js
	firebase deploy

.PHONY: docker
docker: ## Build and push Docker image
	$(MAKE) build-prod-sentry
	docker buildx build --platform linux/amd64 -t openmina/frontend:latest .
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
		./dist/frontend/browser
	sentry-cli sourcemaps upload --org openmina-uv --project openmina \
		./dist/frontend/browser

.PHONY: start
start: ## Start the development server
	npm install
	npx ng serve --configuration local --open

.PHONY: start-bundle
start-bundle: ## Serve the built bundle
	serve dist/frontend/browser -s -l 4200

.PHONY: start-dev-mobile
start-dev-mobile: ## Start the development server for mobile (accessible on network)
	npx ng serve --configuration development --host 0.0.0.0

.PHONY: start-development
start-development: ## Start the development server with development configuration
	npx ng serve --configuration development

.PHONY: start-fuzzing
start-fuzzing: ## Start the fuzzing build and serve
	$(MAKE) install-deps
	npx ng build --configuration fuzzing
	$(MAKE) start-bundle

.PHONY: start-local
start-local: ## Start the development server with local configuration
	npx ng serve --configuration local

.PHONY: start-production
start-production: ## Start the development server with production configuration
	npx ng serve --configuration production

.PHONY: start-webnode
start-webnode: ## Start the webnode development server
	npm install
	npx ng serve --configuration webnodelocal --open

.PHONY: start-webnodelocal
start-webnodelocal: ## Start the development server with webnodelocal configuration
	npx ng serve --configuration webnodelocal

.PHONY: test
test: ## Run Cypress tests
	npx cypress open --config baseUrl=http://localhost:4200

.PHONY: test-headless
test-headless: ## Run Cypress tests in headless mode
	npx cypress run --headless --config baseUrl=http://localhost:4200