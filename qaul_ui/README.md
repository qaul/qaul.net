# qaul_ui

## l10n: Adding a new language support:

1. Create a new arb file over `lib/l10n/app_<LANG_CODE>.arb`
    > As per Flutter documentation, `LANG_CODE` should follow the _"preferred value" entry in the [IANA Language Subtag Registry](https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry)_.
    > 
    > Source: [Locale class](https://api.flutter.dev/flutter/dart-ui/Locale-class.html)
1. Copy and paste the contents of `app_en.arb` to your file. 
1. Replace the values assigned to each key with the new translation.
1. Run `flutter run` on any platform target. This will trigger the generation code to create the language class.
    > The generated code can be found at `qaul_ui/.dart_tool/flutter_gen/gen_l10n`
1. On `UserPrefsHelper` (found at `qaul_ui/lib/helpers/user_prefs_helper.dart`), add to the `supportedLocales` getter the locale tag of the language being added.
    > An example would be `const Locale.fromSubtags(languageCode: 'pt')`
1. The language support is already complete. You can test changing the current lang in the app's settings.
1. To update the text displayed in the language dropdown, add an `if` clause with your language code to `describeLocale` (found at `qaul_ui/packages/utils/lib/utils.dart:59`).

## Local configuration steps