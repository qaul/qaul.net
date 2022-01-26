# qaul.net Flutter UI

This folder contains the Flutter UI of the qaul.net app.

## l10n: Adding a new language support:

1. Create a new arb file over `lib/l10n/app_<LANG_CODE>.arb`
    > As per Flutter documentation, `LANG_CODE` should follow the _"preferred value" entry in the [IANA Language Subtag Registry](https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry)_.
    > 
    > Source: [Locale class](https://api.flutter.dev/flutter/dart-ui/Locale-class.html)
1. Copy and paste the contents of `app_en.arb` to your file. 
1. Replace the values assigned to each key with the new translation.
1. Run `flutter run` on any platform target. This will trigger the generation code to create the language class.
    > The generated code can be found at `qaul_ui/.dart_tool/flutter_gen/gen_l10n`
1. The language support is already complete. You can test changing the current lang in the app's settings.

## Local configuration steps

### Fastlane

[Fastlane](https://docs.fastlane.tools) is a set of tools that help streamline the most tedious tasks relating to code signing, version releasing & so on.

To have it installed, you need to have *Bundler* on your machine. It can be installed with:
`gem install bundler`

Then, `cd` into the `ios` & `android` folders and run:
`bundle install`

We do them separately due to them being considered standalone projects from Fastlane's perspective.
You should be in the respective folder when running fastlane related tasks.
To test if it's properly installed run:
`bundle exec fastlane --version`
> Every command you run using `fastlane` can be run without `bundle exec` at the beginning, but using
> bundler explicitly speeds up the execution time, so it's advised.
 
### (iOS-only) Managing XCode certificates & profiles

We use [Match](https://docs.fastlane.tools/actions/match/) to simplify managing all profiles & certificates for iOS.
On a fresh machine, follow these steps to configure your XCode environment:

* Make sure you already have xcode installed by running: `xcode-select --install`
* Request access to the certificates & profiles git repo.
> *Note*: Match expects you to have SSH configured to be used with github.
* Ask for the decryption passphrase
* Make sure you've installed Fastlane for iOS, then run:
  * `fastlane certificates`
