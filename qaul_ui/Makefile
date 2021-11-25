i10n-messages-build:
	flutter pub run intl_translation:extract_to_arb --output-dir=lib/l10n lib/main.dart

.PHONY: docker-run
docker-run:
	echo $(ANDROID_NDK_HOME)
	docker run --rm --user "$(id -u)":"$(id -g)" \
 		-v "$(shell pwd)":/usr/src/libqaul \
 		-v $(shell echo ${ANDROID_NDK_HOME}):/usr/local/android/sdk/ndk \
 		-e "ANDROID_NDK_HOME=/usr/local/android/sdk/ndk" \
 		-w /usr/src/libqaul rust:latest \
 		/bin/bash -c "apt update; apt install -y curl git binutils-arm-linux-gnueabihf gcc-arm-linux-gnueabihf; rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android; cd rust/libqaul && ./build_libqaul_android.sh"

.PHONY: generate-app-icon
generate-app-icon:
	flutter pub get
	flutter pub run flutter_launcher_icons:main
