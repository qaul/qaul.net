import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

class QaulAuthSegmentedList extends StatelessWidget {
  const QaulAuthSegmentedList({super.key, required this.children});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        for (var i = 0; i < children.length; i++) ...[
          ClipRRect(
            borderRadius: _borderRadiusFor(i, children.length),
            child: Material(
              color: qaulAuthRowBackgroundColor(context),
              child: children[i],
            ),
          ),
          if (i < children.length - 1) const SizedBox(height: kQaulAuthItemGap),
        ],
      ],
    );
  }

  BorderRadius _borderRadiusFor(int index, int total) {
    const radius = Radius.circular(kQaulAuthItemRadius);

    if (total == 1) {
      return const BorderRadius.all(radius);
    }

    if (index == 0) {
      return const BorderRadius.vertical(top: radius);
    }

    if (index == total - 1) {
      return const BorderRadius.vertical(bottom: radius);
    }

    return BorderRadius.zero;
  }
}
