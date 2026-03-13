import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(name: 'Default', type: QaulFAB)
Widget buildQaulFabDefaultUseCase(BuildContext context) {
  return const _QaulFabUseCase(size: 52);
}

@widgetbook.UseCase(name: 'Small (chat)', type: QaulFAB)
Widget buildQaulFabSmallUseCase(BuildContext context) {
  return const _QaulFabUseCase(size: 48);
}

class _QaulFabUseCase extends StatefulWidget {
  const _QaulFabUseCase({required this.size});

  final int size;

  @override
  State<_QaulFabUseCase> createState() => _QaulFabUseCaseState();
}

class _QaulFabUseCaseState extends State<_QaulFabUseCase> {
  var _darkMode = false;
  var _pressCount = 0;

  @override
  Widget build(BuildContext context) {
    final theme = _darkMode ? ThemeData.dark() : ThemeData.light();

    return Theme(
      data: theme,
      child: Material(
        child: SizedBox.expand(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              SwitchListTile(
                value: _darkMode,
                onChanged: (value) => setState(() => _darkMode = value),
                title: const Text('Dark mode'),
              ),
              const SizedBox(height: 24),
              QaulFAB(
                size: widget.size,
                svgAsset: 'assets/icons/public-filled.svg',
                package: 'qaul_components',
                heroTag: 'fabUseCase',
                tooltip: 'Create public post',
                onPressed: () => setState(() => _pressCount++),
              ),
              const SizedBox(height: 16),
              Text('Pressed $_pressCount times'),
            ],
          ),
        ),
      ),
    );
  }
}
