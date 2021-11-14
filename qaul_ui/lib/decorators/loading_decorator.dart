import 'package:flutter/material.dart';

class LoadingDecorator extends StatelessWidget {
  const LoadingDecorator({
    Key? key,
    required this.child,
    this.isLoading = false,
  }) : super(key: key);
  final bool isLoading;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        IgnorePointer(ignoring: isLoading, child: child),
        if (isLoading)
          Positioned.fill(
            child: Container(
              color: Colors.black26,
              child: const Center(child: CircularProgressIndicator()),
            ),
          ),
      ],
    );
  }
}
