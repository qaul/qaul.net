part of 'widgets.dart';

class LanguageSelectDropDown extends ConsumerWidget {
  const LanguageSelectDropDown({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return SizedBox(
          width: constraints.constrainWidth(400),
          child: Row(
            children: [
              SvgPicture.asset(
                'assets/icons/language.svg',
                width: 24,
                height: 24,
                color: Theme.of(context).iconTheme.color,
              ),
              const SizedBox(width: 8.0),
              Text(AppLocalizations.of(context)!.language),
              const SizedBox(width: 12.0),
              Expanded(
                child: ValueListenableBuilder(
                  valueListenable: Hive.box(UserPrefsHelper.hiveBoxName).listenable(),
                  builder: (context, box, _) => DropdownButton<Locale?>(
                    isExpanded: true,
                    value: UserPrefsHelper().defaultLocale,
                    items: <Locale?>[null, ...AppLocalizations.supportedLocales].map((value) {
                      return DropdownMenuItem<Locale?>(
                        value: value,
                        child: Text(
                          value == null
                              ? AppLocalizations.of(context)!.useSystemDefaultMessage
                              : _languageName(value),
                        ),
                      );
                    }).toList(),
                    onChanged: (val) => UserPrefsHelper().defaultLocale = val,
                  ),
                ),
              ),
            ],
          ),
        );
      }
    );
  }

  String _languageName(Locale l) => lookupAppLocalizations(l).languageName;
}
