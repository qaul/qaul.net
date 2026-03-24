import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(name: 'Palette', type: QaulColorSheet)
Widget buildColorSheetPaletteUseCase(BuildContext context) {
  final colorSheet = QaulColorSheet(Theme.of(context).brightness);
  final isDark = colorSheet.brightness == Brightness.dark;
  final textColor = isDark ? Colors.white : Colors.black;

  String hex(Color color) =>
      color.toARGB32().toRadixString(16).padLeft(8, '0').toUpperCase();

  return Material(
    color: colorSheet.background,
    child: Padding(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _ColorSwatch(
            label: 'surfaceContainer',
            color: colorSheet.surfaceContainer,
            textColor: textColor,
            borderColor: isDark ? Colors.white24 : Colors.black12,
            hex: hex,
          ),
        ],
      ),
    ),
  );
}

class _ColorSwatch extends StatelessWidget {
  const _ColorSwatch({
    required this.label,
    required this.color,
    required this.textColor,
    required this.borderColor,
    required this.hex,
  });

  final String label;
  final Color color;
  final Color textColor;
  final Color borderColor;
  final String Function(Color) hex;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 220,
      height: 88,
      decoration: BoxDecoration(
        color: color,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: borderColor),
      ),
      child: Padding(
        padding: const EdgeInsets.all(10),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(
              label,
              style: TextStyle(
                color: textColor,
                fontWeight: FontWeight.w600,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              '0x${hex(color)}',
              style: TextStyle(color: textColor),
            ),
          ],
        ),
      ),
    );
  }
}
