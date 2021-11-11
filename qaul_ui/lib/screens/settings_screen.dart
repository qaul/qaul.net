import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:qaul_ui/widgets/language_select_dropdown.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context);
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          tooltip: l18ns!.backButtonTooltip,
          icon: const Icon(Icons.arrow_back_ios_rounded),
          onPressed: () => Navigator.pop(context),
        ),
        title: Row(
          children: [
            const Icon(Icons.settings),
            const SizedBox(width: 8),
            Text(l18ns.settings),
          ],
        ),
      ),
      body: SingleChildScrollView(
        child: Padding(
          padding:
              MediaQuery.of(context).viewPadding.copyWith(left: 20, right: 20),
          child: Column(
            children: const [
              LanguageSelectDropDown(),
              SizedBox(height: 20),
              _ThemeSelectDropDown(),
              SizedBox(height: 80),
              _InternetNodesTable(),
            ],
          ),
        ),
      ),
    );
  }
}

class _ThemeSelectDropDown extends StatelessWidget {
  const _ThemeSelectDropDown({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context);
    return Row(
      children: [
        const Icon(Icons.palette_outlined),
        const SizedBox(width: 8.0),
        Text(l18ns!.theme),
        const SizedBox(width: 32.0),
        Expanded(
          child: ValueListenableBuilder<AdaptiveThemeMode>(
            valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
            builder: (_, mode, child) {
              var isDark = mode == AdaptiveThemeMode.dark;
              return DropdownButton<bool>(
                isExpanded: true,
                value: !isDark,
                items: [
                  DropdownMenuItem<bool>(
                    value: true,
                    child: Text(l18ns.lightTheme),
                  ),
                  DropdownMenuItem<bool>(
                    value: false,
                    child: Text(l18ns.darkTheme),
                  ),
                ],
                onChanged: (choseLightTheme) {
                  if (choseLightTheme == null) return;
                  choseLightTheme
                      ? AdaptiveTheme.of(context).setLight()
                      : AdaptiveTheme.of(context).setDark();
                },
              );
            },
          ),
        ),
      ],
    );
  }
}

class _InternetNodesTable extends StatelessWidget {
  const _InternetNodesTable();

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context);
    return Column(
      children: [
        Row(
          children: [
            const Icon(CupertinoIcons.globe),
            const SizedBox(width: 8.0),
            Text(l18ns!.internetNodes),
          ],
        ),
        const SizedBox(height: 8.0),
        Table(
          border: TableBorder.all(),
          defaultVerticalAlignment: TableCellVerticalAlignment.middle,
          children: <TableRow>[
            TableRow(
              children: <Widget>[
                Container(
                  height: 32,
                  alignment: Alignment.center,
                  child: Text(l18ns.address),
                ),
                Container(
                  height: 32,
                  alignment: Alignment.center,
                  child: Text(l18ns.name),
                ),
              ],
            ),
            TableRow(
              decoration: const BoxDecoration(
                color: Colors.grey,
              ),
              children: <Widget>[
                Container(height: 64),
                Container(height: 64),
              ],
            ),
          ],
        ),
        const SizedBox(height: 12.0),
        Row(
          children: [
            IconButton(
              icon: const Icon(Icons.add),
              splashRadius: 24,
              onPressed: () => ScaffoldMessenger.of(context)
                ..clearSnackBars()
                ..showSnackBar(const SnackBar(
                  content: Text('This will add a node'),
                )),
            ),
            const SizedBox(width: 12.0),
            Text(l18ns.addNodeCTA),
          ],
        ),
      ],
    );
  }
}
