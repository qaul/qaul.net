part of 'widgets.dart';

class ThemeSelectDropdown extends StatelessWidget {
  const ThemeSelectDropdown({super.key});

  @override
  Widget build(BuildContext context) {
    return SettingsSection(
      name: AppLocalizations.of(context)!.theme,
      icon: const FaIcon(FontAwesomeIcons.palette),
      content: const _ThemeDropdown(),
    );
  }
}

class _ThemeDropdown extends StatelessWidget {
  const _ThemeDropdown();

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return ValueListenableBuilder<AdaptiveThemeMode>(
      valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
      builder: (_, mode, child) {
        return DropdownBuilder<AdaptiveThemeMode>(
          value: mode,
          itemsLength: AdaptiveThemeMode.values.length,
          itemBuilder: (context, i) {
            final val = AdaptiveThemeMode.values[i];
            var label = '';
            switch (val) {
              case AdaptiveThemeMode.light:
                label = l10n.lightTheme;
                break;
              case AdaptiveThemeMode.dark:
                label = l10n.darkTheme;
                break;
              case AdaptiveThemeMode.system:
                label = l10n.useSystemDefaultMessage;
                break;
            }

            return DropdownMenuItem<AdaptiveThemeMode>(
              value: val,
              child: Text(label),
            );
          },
          onChanged: (chosenMode) {
            switch (chosenMode) {
              case AdaptiveThemeMode.light:
                AdaptiveTheme.of(context).setLight();
                break;
              case AdaptiveThemeMode.dark:
                AdaptiveTheme.of(context).setDark();
                break;
              case AdaptiveThemeMode.system:
              default:
                AdaptiveTheme.of(context).setSystem();
                break;
            }
          },
        );
      },
    );
  }
}
