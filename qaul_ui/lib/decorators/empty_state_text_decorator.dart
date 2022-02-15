import 'package:flutter/material.dart';

class EmptyStateTextDecorator extends StatelessWidget {
  const EmptyStateTextDecorator(
    this.text, {
    Key? key,
    required this.child,
    this.isEmpty = false,
  }) : super(key: key);
  final String text;
  final bool isEmpty;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context).textTheme;
    return Stack(
      children: [
        child,
        if (isEmpty)
          Center(
            child: IgnorePointer(
              child: Text(
                text,
                style: theme.bodyText1!
                    .copyWith(color: Theme.of(context).disabledColor),
              ),
            ),
          ),
      ],
    );
  }
}
