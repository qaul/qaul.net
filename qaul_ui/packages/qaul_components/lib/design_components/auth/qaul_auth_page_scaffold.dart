import 'package:flutter/material.dart';

import 'qaul_auth_tokens.dart';

const double _kAuthHeaderHeight = 82;
const double _kAuthHeaderDividerWidth = 0.5;
const double _kAuthHeaderHorizontalPadding = 12;
const double _kAuthHeaderButtonSize = 48;

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
      backgroundColor: qaulAuthBackgroundColor(context),
      body: SafeArea(
        child: Column(
          children: [
            if (showHeader) const _QaulAuthHeader(),
            Expanded(
              child: Center(
                child: ConstrainedBox(
                  constraints: const BoxConstraints(maxWidth: 420),
                  child: child,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _QaulAuthHeader extends StatelessWidget {
  const _QaulAuthHeader();

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: qaulAuthBackgroundColor(context),
        border: Border(
          bottom: BorderSide(
            color: qaulAuthHeaderDividerColor(context),
            width: _kAuthHeaderDividerWidth,
          ),
        ),
      ),
      child: Material(
        color: Colors.transparent,
        child: SizedBox(
          height: _kAuthHeaderHeight,
          child: IconTheme(
            data: IconThemeData(
              color: qaulAuthSecondaryTextColor(context),
              size: 30,
            ),
            child: Padding(
              padding: const EdgeInsets.symmetric(
                horizontal: _kAuthHeaderHorizontalPadding,
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  SizedBox.square(
                    dimension: _kAuthHeaderButtonSize,
                    child: Center(
                      child: IconButton(
                        tooltip: MaterialLocalizations.of(
                          context,
                        ).backButtonTooltip,
                        onPressed: () => Navigator.maybePop(context),
                        padding: EdgeInsets.zero,
                        constraints: const BoxConstraints.tightFor(
                          width: _kAuthHeaderButtonSize,
                          height: _kAuthHeaderButtonSize,
                        ),
                        icon: const Icon(Icons.arrow_back_rounded),
                      ),
                    ),
                  ),
                  const Spacer(),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
