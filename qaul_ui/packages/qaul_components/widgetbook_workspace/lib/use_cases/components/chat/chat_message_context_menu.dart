import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../support/widgetbook_preview.dart';

List<ChatMessageQuickReaction> _reactions() => [
  ChatMessageQuickReaction(
    child: const Text('❤️', style: TextStyle(fontSize: 27)),
    semanticLabel: 'Love',
    onPressed: () {},
  ),
  ChatMessageQuickReaction(
    child: const Text('👍', style: TextStyle(fontSize: 27)),
    semanticLabel: 'Like',
    onPressed: () {},
  ),
  ChatMessageQuickReaction(
    child: const Text('🔥', style: TextStyle(fontSize: 27)),
    semanticLabel: 'Fire',
    onPressed: () {},
  ),
];

Widget _preview(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: ChatMessageContextMenu(
      quickReactions: _reactions(),
      onAddReaction: () {},
      onReply: () {},
      onForward: () {},
      onEdit: () {},
      onDismiss: () {},
    ),
  );
}

@widgetbook.UseCase(name: 'Dark', type: ChatMessageContextMenu)
Widget buildDarkContextMenuUseCase(BuildContext context) {
  return Theme(
    data: QaulAppTheme.dark,
    child: const Builder(builder: _preview),
  );
}

@widgetbook.UseCase(name: 'Light', type: ChatMessageContextMenu)
Widget buildLightContextMenuUseCase(BuildContext context) {
  return Theme(
    data: QaulAppTheme.light,
    child: const Builder(builder: _preview),
  );
}

@widgetbook.UseCase(
  name: 'Disabled and hidden actions',
  type: ChatMessageContextMenu,
)
Widget buildRestrictedContextMenuUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: ChatMessageContextMenu(
      quickReactions: _reactions(),
      onReply: () {},
      showForward: false,
      editEnabled: false,
      onEdit: () {},
      onDismiss: () {},
    ),
  );
}
