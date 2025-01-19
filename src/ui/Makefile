.PHONY: update-htmx
update-htmx:
	@mkdir -p ./public/static
	@wget https://unpkg.com/htmx.org/dist/htmx.min.js -O ./public/static/htmx.min.js
	@wget https://unpkg.com/htmx.org/dist/ext/json-enc.js -O ./public/static/json-enc.js

.PHONY: ui
ui:
	@npm run build

.PHONY: setup-ui
setup-ui:
	@npm install
	@make update-htmx ui

.PHONY: clean
clean: 
	@rm /public/static