part of 'widgets.dart';

class LanguageSelectDropDown extends ConsumerWidget {
  const LanguageSelectDropDown({
    super.key,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return SettingsSection(
      name: AppLocalizations.of(context)!.language,
      icon: SvgPicture.asset(
        'assets/icons/language.svg',
        width: 24,
        height: 24,
        colorFilter: ColorFilter.mode(
          Theme.of(context).iconTheme.color!,
          BlendMode.srcATop,
        ),
      ),
      content: const _LanguageDropdown(),
    );
  }
}

class _LanguageDropdown extends StatelessWidget {
  const _LanguageDropdown();

  String _languageName(Locale l) => lookupAppLocalizations(l).languageName;

  @override
  Widget build(BuildContext context) {
    final items = <Locale?>[null, ...AppLocalizations.supportedLocales];

    return ValueListenableBuilder(
      valueListenable: Hive.box(UserPrefsHelper.hiveBoxName).listenable(),
      builder: (context, box, _) {
        return DropdownBuilder<Locale?>(
          value: UserPrefsHelper().defaultLocale,
          itemsLength: items.length,
          onChanged: (val) => UserPrefsHelper().defaultLocale = val,
          itemBuilder: (c, i) {
            final value = items[i];
            return DropdownMenuItem<Locale?>(
              value: value,
              child: Text(
                value == null
                    ? AppLocalizations.of(context)!.useSystemDefaultMessage
                    : _languageName(value),
              ),
            );
          },
        );
      },
    );
  }
}
