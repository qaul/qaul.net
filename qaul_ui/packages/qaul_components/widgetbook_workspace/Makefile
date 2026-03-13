.PHONY: get generate watch run clean

get:
	flutter pub get

generate:
	dart run build_runner build --delete-conflicting-outputs

watch:
	dart run build_runner watch --delete-conflicting-outputs

run:
	flutter run -d chrome

clean:
	flutter clean
	dart run build_runner clean
