import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';

class LoadingDecorator extends StatelessWidget {
  const LoadingDecorator({
    super.key,
    required this.child,
    this.isLoading = false,
    this.backgroundColor = Colors.black26,
    this.loadingPadding = EdgeInsets.zero,
  });
  final bool isLoading;
  final Color backgroundColor;
  final EdgeInsetsGeometry loadingPadding;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        IgnorePointer(
          ignoring: isLoading,
          child: child,
        ),
        if (isLoading)
          Positioned.fill(
            child: Container(
              color: backgroundColor,
              child: Padding(
                padding: loadingPadding,
                child: const QaulLoadingIndicator(),
              ),
            ),
          ),
      ],
    );
  }
}
