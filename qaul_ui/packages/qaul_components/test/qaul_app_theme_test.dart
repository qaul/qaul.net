import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  test('dark theme uses the color sheet page background', () {
    final theme = QaulAppTheme.dark;
    final pageBackground = const QaulColorSheet(Brightness.dark).background;

    expect(theme.scaffoldBackgroundColor, pageBackground);
    expect(theme.scaffoldBackgroundColor, Colors.black);
    expect(theme.colorScheme.surface, isNot(pageBackground));
  });

  testWidgets('page background helpers and FAB use the theme default', (
    tester,
  ) async {
    late BuildContext context;

    await tester.pumpWidget(
      MaterialApp(
        theme: QaulAppTheme.dark,
        home: Builder(
          builder: (builderContext) {
            context = builderContext;
            return Scaffold(
              body: QaulFAB(
                svgAsset: 'assets/icons/public-filled.svg',
                package: 'qaul_components',
                onPressed: () {},
              ),
            );
          },
        ),
      ),
    );

    final pageBackground = Theme.of(context).scaffoldBackgroundColor;
    final fab = tester.widget<FloatingActionButton>(
      find.byType(FloatingActionButton),
    );

    expect(qaulSettingsBackgroundColor(context), pageBackground);
    expect(qaulAuthBackgroundColor(context), pageBackground);
    expect(fab.backgroundColor, pageBackground);
  });
}
