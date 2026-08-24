import 'package:flutter/material.dart';

class QaulSettingsMenuContent extends StatelessWidget {
  const QaulSettingsMenuContent({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) => child;
}

class QaulSettingsDetailContent extends StatelessWidget {
  const QaulSettingsDetailContent({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) => child;
}

class QaulSettingsPaddedContent extends StatelessWidget {
  const QaulSettingsPaddedContent({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(28, 4, 28, 0),
      child: child,
    );
  }
}
