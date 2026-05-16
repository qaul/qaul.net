import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';

QaulColorSheet widgetbookColorSheet(BuildContext context) =>
    QaulColorSheet(Theme.of(context).brightness);

/// Chat canvas / message list background from the design system.
Color widgetbookChatCanvasColor(BuildContext context) =>
    widgetbookColorSheet(context).background;

/// Area below chrome (header, nav) in previews.
Color widgetbookChatSurfaceColor(BuildContext context) =>
    widgetbookColorSheet(context).surfaceContainer;

TextStyle widgetbookMetaLabelStyle(BuildContext context) {
  final theme = Theme.of(context);
  return TextStyle(
    fontSize: 12,
    height: 1.2,
    color: theme.colorScheme.onSurface.withValues(alpha: 0.55),
  );
}

/// Fills the Widgetbook viewport with themed chat canvas; scrolls when needed.
Widget widgetbookFullScreenChatPreview(
  BuildContext context,
  Widget timeline,
) {
  return SizedBox.expand(
    child: ColoredBox(
      color: widgetbookChatCanvasColor(context),
      child: SingleChildScrollView(child: timeline),
    ),
  );
}

/// Header use case: shell header over themed chat surface.
Widget widgetbookTopChromeFrame(BuildContext context, Widget header) {
  final sheet = widgetbookColorSheet(context);
  return Material(
    color: sheet.background,
    child: ColoredBox(
      color: sheet.surfaceContainer,
      child: Column(
        children: [
          header,
          const Expanded(child: SizedBox.expand()),
        ],
      ),
    ),
  );
}

/// Single-component preview on the chat canvas.
Widget widgetbookChatComponentFrame(
  BuildContext context, {
  required Widget child,
  EdgeInsetsGeometry padding = const EdgeInsets.all(16),
  Alignment alignment = Alignment.center,
}) {
  return SizedBox.expand(
    child: ColoredBox(
      color: widgetbookChatCanvasColor(context),
      child: Padding(
        padding: padding,
        child: Align(alignment: alignment, child: child),
      ),
    ),
  );
}
