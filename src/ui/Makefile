.PHONY: update-htmx
update-htmx:
	@mkdir -p ./public/static
	@wget https://unpkg.com/htmx.org/dist/htmx.min.js -O ./public/static/htmx.min.js
	@wget https://unpkg.com/htmx.org/dist/ext/json-enc.js -O ./public/static/json-enc.js

.PHONY: nvm
nvm:
	. ${NVM_DIR}/nvm.sh && nvm use

.PHONY: ui
ui:
	@make nvm
	@npm run build

.PHONY: setup-ui
setup-ui:
	@make nvm
	@npm install
	@make update-htmx ui

.PHONY: clean
clean: 
	@rm /public/static