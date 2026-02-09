.PHONY: publish-all sync-versions

sync-versions:
	@chmod +x ./sync_versions.sh
	@./sync_versions.sh

publish-submodule:
	@echo "Checking current version against crates.io for submodule $(SUBMODULE)..."
	@cd $(SUBMODULE) && \
	CURRENT_VERSION=$$(grep -m 1 "^version =" Cargo.toml | cut -d '"' -f 2); \
	CRATE_NAME=$$(grep -m 1 "^name =" Cargo.toml | cut -d '"' -f 2); \
	echo "Current crate: $$CRATE_NAME, version: $$CURRENT_VERSION"; \
	if cargo search "$$CRATE_NAME" --limit 1 | grep -q "$$CRATE_NAME = \"$$CURRENT_VERSION\""; then \
		echo "Version $$CURRENT_VERSION already exists on crates.io, skipping publish for $$CRATE_NAME."; \
	else \
		echo "Local version ($$CURRENT_VERSION) not found on crates.io, publishing $$CRATE_NAME..."; \
		cargo publish --all-features --allow-dirty; \
	fi

publish-main:
	@echo "Checking current version against crates.io for main repository..."
	@CURRENT_VERSION=$$(grep -m 1 "^version =" Cargo.toml | cut -d '"' -f 2); \
	CRATE_NAME=$$(grep -m 1 "^name =" Cargo.toml | cut -d '"' -f 2); \
	echo "Current crate: $$CRATE_NAME, version: $$CURRENT_VERSION"; \
	if cargo search "$$CRATE_NAME" --limit 1 | grep -q "$$CRATE_NAME = \"$$CURRENT_VERSION\""; then \
		echo "Version $$CURRENT_VERSION already exists on crates.io, skipping publish for $$CRATE_NAME."; \
	else \
		echo "Local version ($$CURRENT_VERSION) not found on crates.io, publishing $$CRATE_NAME..."; \
		cargo publish --all-features --allow-dirty; \
	fi

publish-all:
	@$(MAKE) sync-versions
	@echo "Publishing all packages with spider-lib (main repo) last..."
	@$(MAKE) SUBMODULE="spider-util" publish-submodule
	@$(MAKE) SUBMODULE="spider-macro" publish-submodule
	@$(MAKE) SUBMODULE="spider-middleware" publish-submodule
	@$(MAKE) SUBMODULE="spider-pipeline" publish-submodule
	@$(MAKE) SUBMODULE="spider-downloader" publish-submodule
	@$(MAKE) SUBMODULE="spider-core" publish-submodule
	@$(MAKE) publish-main
	@echo "Publishing complete!"