import 'package:flutter/material.dart';

import '../../styles/qaul_color_sheet.dart';

const Color kQaulPageHeaderControlColor = Color(0xFF999999);
const double kQaulPageHeaderHeight = 82;
const double _kDividerWidth = 0.5;
const double _kHorizontalPadding = 12;
const double _kBackVisualGap = 4;
const double _kVisualTitleGap = 12;
const double _kControlIconSize = 30;

Color _headerShellColor(ThemeData theme) {
  return QaulColorSheet(theme.brightness).background;
}

Color _headerDividerColor(ThemeData theme) {
  return QaulColorSheet(theme.brightness).chatHeaderDivider;
}

Color _headerTextColor(ThemeData theme) => theme.colorScheme.onSurface;

BoxShadow _headerShadow(ThemeData theme) {
  return theme.brightness == Brightness.dark
      ? const BoxShadow(
          offset: Offset(0, 10),
          blurRadius: 7,
          color: Color(0x66000000),
        )
      : const BoxShadow(blurRadius: 5, color: Color(0x33000000));
}

class QaulPageHeader extends StatelessWidget implements PreferredSizeWidget {
  const QaulPageHeader({
    super.key,
    required this.title,
    this.leadingVisual,
    this.subtitle,
    this.actions = const [],
    this.onBackPressed,
    this.backButtonTooltip,
    this.showBackButton = true,
  });

  final String title;
  final Widget? leadingVisual;
  final String? subtitle;
  final List<Widget> actions;
  final VoidCallback? onBackPressed;
  final String? backButtonTooltip;
  final bool showBackButton;

  @override
  Size get preferredSize => const Size.fromHeight(kQaulPageHeaderHeight);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final hasSubtitle = subtitle != null && subtitle!.isNotEmpty;
    final titleStyle = (theme.textTheme.titleMedium ?? const TextStyle())
        .copyWith(
          fontFamily: 'Roboto',
          fontSize: 20,
          fontWeight: FontWeight.w400,
          height: 1.2,
          letterSpacing: 1,
          color: _headerTextColor(theme),
        );
    final subtitleStyle = (theme.textTheme.bodySmall ?? const TextStyle())
        .copyWith(
          fontFamily: 'Roboto',
          fontSize: 11,
          fontWeight: FontWeight.w400,
          height: 1.2,
          letterSpacing: 0.5,
          color: _headerTextColor(theme),
        );

    return SafeArea(
      bottom: false,
      child: DecoratedBox(
        decoration: BoxDecoration(
          color: _headerShellColor(theme),
          border: Border(
            bottom: BorderSide(
              color: _headerDividerColor(theme),
              width: _kDividerWidth,
            ),
          ),
          boxShadow: [_headerShadow(theme)],
        ),
        child: Material(
          color: Colors.transparent,
          child: SizedBox(
            height: kQaulPageHeaderHeight,
            child: Padding(
              padding: const EdgeInsets.symmetric(
                horizontal: _kHorizontalPadding,
              ),
              child: IconTheme(
                data: const IconThemeData(
                  color: kQaulPageHeaderControlColor,
                  size: _kControlIconSize,
                ),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    if (showBackButton) ...[
                      IconButton(
                        tooltip:
                            backButtonTooltip ??
                            MaterialLocalizations.of(
                              context,
                            ).backButtonTooltip,
                        onPressed:
                            onBackPressed ?? () => Navigator.maybePop(context),
                        icon: const Icon(Icons.arrow_back_rounded),
                      ),
                      const SizedBox(width: _kBackVisualGap),
                    ],
                    if (leadingVisual != null) ...[
                      leadingVisual!,
                      const SizedBox(width: _kVisualTitleGap),
                    ],
                    Expanded(
                      child: hasSubtitle
                          ? Column(
                              mainAxisAlignment: MainAxisAlignment.center,
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  title,
                                  maxLines: 1,
                                  overflow: TextOverflow.ellipsis,
                                  style: titleStyle,
                                ),
                                const SizedBox(height: 4),
                                Text(
                                  subtitle!,
                                  maxLines: 1,
                                  overflow: TextOverflow.ellipsis,
                                  style: subtitleStyle,
                                ),
                              ],
                            )
                          : Align(
                              alignment: Alignment.centerLeft,
                              child: Text(
                                title,
                                maxLines: 1,
                                overflow: TextOverflow.ellipsis,
                                style: titleStyle,
                              ),
                            ),
                    ),
                    if (actions.isNotEmpty) ...actions,
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
