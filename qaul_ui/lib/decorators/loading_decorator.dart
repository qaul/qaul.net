import 'package:flutter/material.dart';

import '../widgets/widgets.dart';

class LoadingDecorator extends StatelessWidget {
  const LoadingDecorator({
    super.key,
    required this.child,
    this.isLoading = false,
    this.backgroundColor = Colors.black26,
    this.hideChildWhenLoading = false,
    this.loadingPadding = EdgeInsets.zero,
  });
  final bool isLoading;
  final Color backgroundColor;
  final bool hideChildWhenLoading;
  final EdgeInsetsGeometry loadingPadding;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        IgnorePointer(
          ignoring: isLoading,
          child: hideChildWhenLoading
              ? Visibility(
                  visible: !isLoading,
                  maintainState: true,
                  maintainAnimation: true,
                  maintainSize: true,
                  child: child,
                )
              : child,
        ),
        if (isLoading)
          Positioned.fill(
            child: Container(
              color: backgroundColor,
              child: Padding(
                padding: loadingPadding,
                child: const LoadingIndicator(),
              ),
            ),
          ),
      ],
    );
  }
}
