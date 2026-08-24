import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

const double _kAuthHeaderHeight = 82;
const double _kAuthHeaderDividerWidth = 0.5;

class QaulAuthPageScaffold extends StatelessWidget {
  const QaulAuthPageScaffold({
    super.key,
    required this.child,
    this.showHeader = true,
  });

  final Widget child;
  final bool showHeader;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: kQaulAuthBackgroundColor,
      appBar: showHeader ? const _QaulAuthHeader() : null,
      body: SafeArea(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 420),
            child: child,
          ),
        ),
      ),
    );
  }
}

class _QaulAuthHeader extends StatelessWidget implements PreferredSizeWidget {
  const _QaulAuthHeader();

  @override
  Size get preferredSize => const Size.fromHeight(_kAuthHeaderHeight);

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: const BoxDecoration(
        color: kQaulAuthBackgroundColor,
        border: Border(
          bottom: BorderSide(
            color: Color(0xFF333333),
            width: _kAuthHeaderDividerWidth,
          ),
        ),
      ),
      child: Material(
        color: Colors.transparent,
        child: SizedBox(
          height: _kAuthHeaderHeight,
          child: IconTheme(
            data: const IconThemeData(
              color: kQaulAuthSecondaryTextColor,
              size: 30,
            ),
            child: Row(
              children: [
                const SizedBox(width: 12),
                IconButton(
                  tooltip: MaterialLocalizations.of(context).backButtonTooltip,
                  onPressed: () => Navigator.maybePop(context),
                  icon: const Icon(Icons.arrow_back_rounded),
                ),
                const Spacer(),
                const Icon(Icons.more_vert),
                const SizedBox(width: 24),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
