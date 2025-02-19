import 'package:flutter/material.dart';

import '../widgets/widgets.dart';

class LoadingDecorator extends StatelessWidget {
  const LoadingDecorator({
    super.key,
    required this.child,
    this.isLoading = false,
    this.backgroundColor = Colors.black26,
  });
  final bool isLoading;
  final Color backgroundColor;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        IgnorePointer(ignoring: isLoading, child: child),
        if (isLoading)
          Positioned.fill(
            child: Container(
              color: backgroundColor,
              child: const LoadingIndicator(),
            ),
          ),
      ],
    );
  }
}
