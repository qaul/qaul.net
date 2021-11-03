import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:qaul_ui/widgets/language_select_dropdown.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          tooltip: 'Back',
          icon: const Icon(Icons.arrow_back_ios_rounded),
          onPressed: () => Navigator.pop(context),
        ),
        title: Row(
          children: const [
            Icon(Icons.settings),
            SizedBox(width: 8),
            Text('Settings'),
          ],
        ),
      ),
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 20.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              SizedBox(height: MediaQuery.of(context).size.height * .2),
              const LanguageSelectDropDown(),
              const SizedBox(height: 20),
              const _ThemeSelectDropDown(),
              const SizedBox(height: 120),
              const _InternetNodesTable(),
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
    return Row(
      children: [
        const Icon(Icons.palette_outlined),
        const SizedBox(width: 8.0),
        const Text('Theme'),
        const SizedBox(width: 32.0),
        Expanded(
          child: ValueListenableBuilder<AdaptiveThemeMode>(
            valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
            builder: (_, mode, child) {
              var isDark = mode == AdaptiveThemeMode.dark;
              return DropdownButton<bool>(
                isExpanded: true,
                value: !isDark,
                items: const [
                  DropdownMenuItem<bool>(
                    value: true,
                    child: Text('Light theme'),
                  ),
                  DropdownMenuItem<bool>(
                    value: false,
                    child: Text('Dark theme'),
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
    return Column(
      children: [
        Row(
          children: const [
            Icon(CupertinoIcons.globe),
            SizedBox(width: 8.0),
            Text('Internet Nodes'),
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
                  child: const Text('Address'),
                ),
                Container(
                  height: 32,
                  alignment: Alignment.center,
                  child: const Text('Name'),
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
            const Text('Add internet node'),
          ],
        ),
      ],
    );
  }
}
