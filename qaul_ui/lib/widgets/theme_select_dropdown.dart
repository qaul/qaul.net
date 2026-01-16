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

    return ValueListenableBuilder<ThemeMode>(
      valueListenable: UserPrefsHelper().themeModeNotifier,
      builder: (context, mode, child) {
        return DropdownBuilder<ThemeMode>(
          value: mode,
          itemsLength: ThemeMode.values.length,
          itemBuilder: (context, i) {
            final val = ThemeMode.values[i];
            var label = '';
            switch (val) {
              case ThemeMode.light:
                label = l10n.lightTheme;
                break;
              case ThemeMode.dark:
                label = l10n.darkTheme;
                break;
              case ThemeMode.system:
                label = l10n.useSystemDefaultMessage;
                break;
            }

            return DropdownMenuItem<ThemeMode>(
              value: val,
              child: Text(label),
            );
          },
          onChanged: (chosenMode) async {
            if (chosenMode == null) return;
            await UserPrefsHelper().setThemeMode(chosenMode);
          },
        );
      },
    );
  }
}
