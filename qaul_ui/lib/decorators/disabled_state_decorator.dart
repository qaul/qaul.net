import 'package:flutter/material.dart';

class DisabledStateDecorator extends StatelessWidget {
  const DisabledStateDecorator({
    super.key,
    required this.child,
    this.isDisabled = false,
    this.ignorePointer = true,
  });
  final bool isDisabled;
  final bool ignorePointer;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return ColorFiltered(
      colorFilter: ColorFilter.mode(
        isDisabled ? Colors.grey : Colors.white,
        BlendMode.modulate,
      ),
      child: DefaultTextStyle.merge(
        style: !isDisabled ? null : DefaultTextStyle.of(context).style.copyWith(color: Colors.grey),
        child: IgnorePointer(ignoring: ignorePointer && isDisabled, child: child),
      ),
    );
  }
}
