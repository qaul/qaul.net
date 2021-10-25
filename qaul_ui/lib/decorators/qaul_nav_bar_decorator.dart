import 'dart:math' as math;

import 'package:badges/badges.dart';
import 'package:flutter/material.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';

class QaulNavBarDecorator extends StatefulWidget {
  const QaulNavBarDecorator({Key? key, required this.child}) : super(key: key);
  final Widget child;

  @override
  State<QaulNavBarDecorator> createState() => _QaulNavBarDecoratorState();
}

class _QaulNavBarDecoratorState extends State<QaulNavBarDecorator> {
  static final _overflowMenuOptions = {'Settings', 'About'};

  void _handleClick(String value) {
    switch (value) {
      case 'Settings':
        Navigator.pushNamed(context, NavigationHelper.settings);
        break;
      case 'About':
        Navigator.pushNamed(context, NavigationHelper.about);
        break;
    }
  }

  @override
  Widget build(BuildContext context) {
    return OrientationBuilder(
      builder: (context, orientation) {
        return Stack(
          alignment: orientation == Orientation.portrait
              ? AlignmentDirectional.topCenter
              : AlignmentDirectional.topStart,
          children: [
            widget.child,
            orientation == Orientation.portrait
                ? _buildHorizontalBar(context)
                : _buildVerticalBar(context),
          ],
        );
      },
    );
  }

  List<Widget> get _tabBarContent {
    return [
      const QaulNavBarItem(_TabType.account),
      const QaulNavBarItem(_TabType.feed),
      const QaulNavBarItem(_TabType.users),
      const QaulNavBarItem(_TabType.chat),
      const QaulNavBarItem(_TabType.network),
      const SizedBox(width: 40, height: 40),
      PopupMenuButton<String>(
        onSelected: _handleClick,
        iconSize: 30,
        itemBuilder: (BuildContext context) {
          return _overflowMenuOptions.map((String choice) {
            return PopupMenuItem<String>(
              value: choice,
              child: Text(choice),
            );
          }).toList();
        },
      ),
    ];
  }

  Widget _buildHorizontalBar(BuildContext context) {
    return FractionallySizedBox(
      heightFactor: 0.12,
      child: _barBackground(
        context,
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: _tabBarContent,
        ),
      ),
    );
  }

  Widget _buildVerticalBar(BuildContext context) {
    return FractionallySizedBox(
      widthFactor: 0.1,
      child: _barBackground(
        context,
        Column(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: [
            SizedBox(height: MediaQuery.of(context).viewPadding.top),
            ..._tabBarContent,
          ],
        ),
      ),
    );
  }

  Widget _barBackground(BuildContext context, Widget child) => Container(
        alignment: Alignment.bottomCenter,
        color: Theme.of(context).primaryColor,
        padding: const EdgeInsets.all(8.0),
        child: child,
      );
}

enum _TabType { account, feed, users, chat, network }

class QaulNavBarItem extends StatelessWidget {
  const QaulNavBarItem(this.tab, {Key? key}) : super(key: key);
  final _TabType tab;

  @override
  Widget build(BuildContext context) {
    IconData icon;
    switch (tab) {
      case _TabType.account:
        return Badge(
          position: BadgePosition.bottomEnd(bottom: 0, end: 0),
          badgeColor: Colors.greenAccent.shade700,
          child: CircleAvatar(
            child: const Text('BD'),
            backgroundColor: Colors
                .primaries[math.Random().nextInt(Colors.primaries.length)]
                .shade700,
          ),
        );
      case _TabType.feed:
        icon = Icons.tag;
        break;
      case _TabType.users:
        icon = Icons.group;
        break;
      case _TabType.chat:
        icon = Icons.comment;
        break;
      case _TabType.network:
        icon = Icons.public;
        break;
    }
    return IconButton(icon: Icon(icon), onPressed: () {});
  }
}
