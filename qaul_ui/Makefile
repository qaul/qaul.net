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
	flutter test integration_test -d "$$ID" --dart-define=testing_mode=true
