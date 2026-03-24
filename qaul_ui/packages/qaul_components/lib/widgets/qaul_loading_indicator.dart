import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class QaulLoadingIndicator extends StatelessWidget {
  const QaulLoadingIndicator({super.key});

  @override
  Widget build(BuildContext context) {
    final platform = Theme.of(context).platform;
    final isCupertino =
        platform == TargetPlatform.iOS || platform == TargetPlatform.macOS;

    return Center(
      child: isCupertino
          ? const CupertinoActivityIndicator()
          : const CircularProgressIndicator(),
    );
  }
}
