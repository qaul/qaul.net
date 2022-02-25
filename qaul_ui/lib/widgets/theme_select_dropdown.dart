part of 'widgets.dart';

class ThemeSelectDropdown extends StatelessWidget {
  const ThemeSelectDropdown({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context);
    return Row(
      children: [
        const Icon(Icons.palette_outlined),
        const SizedBox(width: 8.0),
        Text(l18ns!.theme),
        const SizedBox(width: 32.0),
        const Expanded(child: _PlatformAwareDropdown()),
      ],
    );
  }
}

class _PlatformAwareDropdown extends PlatformAwareBuilder {
  const _PlatformAwareDropdown({Key? key}) : super(key: key);

  @override
  Widget defaultBuilder(BuildContext context, WidgetRef ref) {
    return ValueListenableBuilder<AdaptiveThemeMode>(
      valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
      builder: (_, mode, child) {
        final l18ns = AppLocalizations.of(context)!;
        return DropdownButton<AdaptiveThemeMode>(
          isExpanded: true,
          value: mode,
          items: [
            DropdownMenuItem<AdaptiveThemeMode>(
              value: AdaptiveThemeMode.system,
              child: Text(l18ns.useSystemDefaultMessage),
            ),
            DropdownMenuItem<AdaptiveThemeMode>(
              value: AdaptiveThemeMode.light,
              child: Text(l18ns.lightTheme),
            ),
            DropdownMenuItem<AdaptiveThemeMode>(
              value: AdaptiveThemeMode.dark,
              child: Text(l18ns.darkTheme),
            ),
          ],
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
