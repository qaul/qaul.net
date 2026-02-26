import 'package:flutter/material.dart';

import '../nav_bar/nav_bar_helper.dart';
import '../nav_bar/widgets/qaul_nav_bar.dart';
import '../widgets/widgets.dart';

class QaulNavBarDecorator extends StatefulWidget {
  const QaulNavBarDecorator({super.key, required this.child});

  final Widget Function(GlobalKey pageViewKey) child;

  @override
  State<QaulNavBarDecorator> createState() => _QaulNavBarDecoratorState();
}

class _QaulNavBarDecoratorState extends State<QaulNavBarDecorator> {
  final _pageViewKey = GlobalKey();

  @override
  Widget build(BuildContext context) {
    return ResponsiveLayout(
      mobileBody: Column(
        children: [
          Expanded(child: widget.child(_pageViewKey)),
          QaulNavBar(
            vertical: false,
            overflowMenuLabels: navBarOverflowMenuLabels(context),
            onOverflowSelected: (option) =>
                handleNavBarOverflowSelected(context, option),
          ),
        ],
      ),
      tabletBody: Row(
        children: [
          QaulNavBar(
            vertical: true,
            overflowMenuLabels: navBarOverflowMenuLabels(context),
            onOverflowSelected: (option) =>
                handleNavBarOverflowSelected(context, option),
          ),
          Expanded(child: widget.child(_pageViewKey)),
        ],
      ),
    );
  }
}
