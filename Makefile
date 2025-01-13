.PHONY: ui
ui:
	@echo "🚀 Building UI components..."
	@cd src/api/ui && make ui
	@echo "✨ UI build completed!"

.PHONY: setup-ui
setup-ui:
	@echo "🔧 Setting up UI environment..."
	@cd src/api/ui && make setup-ui
	@echo "✅ UI setup completed!"

.PHONY: serve
serve:
	@echo "🌐 Starting development server..."
	@echo "📡 Browser-Sync listening on http://localhost:8080..."
	@trap 'kill $$(jobs -p)' EXIT; \
	npx browser-sync start --logLevel "silent" --proxy "localhost:3000" --port 8080 --files "src/" --no-open --no-ui & \
	systemfd --no-pid -s http::3000 -- cargo watch -x 'run serve'

.PHONY: models
models:
	@echo "🚀 Generating models..."
	@mkdir -p models/potion-base-8M
	@wget https://huggingface.co/minishlab/potion-base-8M/resolve/main/onnx/model.onnx?download=true -O models/potion-base-8M/potion-base-8M.onnx
	@wget https://huggingface.co/minishlab/potion-base-8M/resolve/main/tokenizer.json -O models/potion-base-8M/tokenizer.json
	@echo "✨ Models generated!"

.PHONE: deploy
deploy:
	@echo "🚀 Deploying ..."
	@./docker-deploy.sh
	@./deploy.sh
	@echo "✨ Deployment completed!"