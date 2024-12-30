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
	@npx browser-sync start --logLevel "silent" --proxy "localhost:3000" --port 8080 --files "src/" --no-open --no-ui & \
	systemfd --no-pid -s http::3000 -- cargo watch -x 'run serve'
