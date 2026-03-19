import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(name: 'Palette', type: QaulColorSheet)
Widget buildColorSheetPaletteUseCase(BuildContext context) {
  return const _ColorSheetPaletteUseCase();
}

class _ColorSheetPaletteUseCase extends StatefulWidget {
  const _ColorSheetPaletteUseCase();

  @override
  State<_ColorSheetPaletteUseCase> createState() => _ColorSheetPaletteUseCaseState();
}

class _ColorSheetPaletteUseCaseState extends State<_ColorSheetPaletteUseCase> {
  final _colorSheet = QaulColorSheet();

  String _hex(Color color) =>
      color.toARGB32().toRadixString(16).padLeft(8, '0').toUpperCase();

  @override
  Widget build(BuildContext context) {
    final darkMode = _colorSheet.mode == QaulColorMode.dark;
    final surfaceContainerColor = _colorSheet.surfaceContainer;

    return Material(
      color: _colorSheet.background,
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Text(
                  'Dark mode',
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    color: darkMode ? Colors.white : Colors.black,
                  ),
                ),
                const SizedBox(width: 12),
                Switch(
                  value: darkMode,
                  onChanged: (enabled) {
                    setState(() {
                      _colorSheet.mode = enabled
                          ? QaulColorMode.dark
                          : QaulColorMode.light;
                    });
                  },
                ),
              ],
            ),
            const SizedBox(height: 16),
            Container(
              width: 220,
              height: 88,
              decoration: BoxDecoration(
                color: surfaceContainerColor,
                borderRadius: BorderRadius.circular(8),
                border: Border.all(
                  color: darkMode ? Colors.white24 : Colors.black12,
                ),
              ),
              child: Padding(
                padding: const EdgeInsets.all(10),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    const Text(
                      'surfaceContainer',
                      style: TextStyle(
                        color: Colors.black,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      '0x${_hex(surfaceContainerColor)}',
                      style: const TextStyle(color: Colors.black),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
