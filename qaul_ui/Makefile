i10n-messages-build:
	flutter pub run intl_translation:extract_to_arb --output-dir=lib/l10n lib/main.dart

.PHONY: generate-app-icon
generate-app-icon:
	flutter pub get
	flutter pub run flutter_launcher_icons:main

.PHONY: integration-test
integration-test:
	@ID=$$(flutter devices | grep ios | cut -d "•" -f 2 | xargs); \
	applesimutils --byId "$$ID" --bundle net.qaul.app --setPermissions notifications=YES; \
	flutter test integration_test -d "$$ID" --dart-define=testing_mode=true #--update-goldens

# -----------------------
# WidgetBook
# -----------------------
run-widgetbook:
	@cd packages/qaul_components && flutter run -d macos
# -----------------------

.PHONY: sort-dependencies
sort-dependencies:
	@docker run --rm -v $(shell pwd):"/qaul" python:latest /bin/bash -c 'sleep 5; pip install pyyaml && python ./qaul/bin/sort_pubspec_deps.py'
	@echo ""

.PHONY: unused-dependency-check
unused-dependency-check:
	@docker run --rm -v $(shell pwd):"/qaul" python:latest /bin/bash -c 'sleep 5; pip install pyyaml && python ./qaul/bin/check_unused_deps.py'
	@echo ""
