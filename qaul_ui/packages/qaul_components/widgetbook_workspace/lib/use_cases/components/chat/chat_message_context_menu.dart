import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../support/widgetbook_preview.dart';

ChatMessageReactionRow _reactions({bool enabled = true}) {
  return ChatMessageReactionRow(
    enabled: enabled,
    reactions: [
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
    ],
    onAddReaction: () {},
  );
}

List<ChatMessageContextMenuElement> _paginatedElements() => [
  _reactions(),
  ChatMessageContextMenuAction.reply(onPressed: () {}),
  ChatMessageContextMenuAction.forward(onPressed: () {}),
  ChatMessageContextMenuAction.edit(onEdit: () {}),
  ChatMessageContextMenuAction(
    id: 'info',
    label: 'Info',
    iconAsset: ChatMessageContextMenuIcons.info,
    onPressed: () {},
  ),
  ChatMessageContextMenuAction(
    id: 'share',
    label: 'Share',
    iconAsset: ChatMessageContextMenuIcons.share,
    onPressed: () {},
  ),
  ChatMessageContextMenuAction(
    id: 'copy',
    label: 'Copy',
    iconAsset: ChatMessageContextMenuIcons.copy,
    onPressed: () {},
  ),
  ChatMessageContextMenuAction(
    id: 'delete',
    label: 'Delete',
    iconAsset: ChatMessageContextMenuIcons.delete,
    onPressed: () {},
  ),
];

@widgetbook.UseCase(name: 'Default', type: ChatMessageContextMenu)
Widget buildContextMenuUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: ChatMessageContextMenu(elements: _paginatedElements()),
  );
}

@widgetbook.UseCase(
  name: 'Disabled and hidden elements',
  type: ChatMessageContextMenu,
)
Widget buildRestrictedContextMenuUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: ChatMessageContextMenu(
      elements: [
        _reactions(enabled: false),
        ChatMessageContextMenuAction.reply(onPressed: () {}),
        ChatMessageContextMenuAction.forward(hidden: true, onPressed: () {}),
        ChatMessageContextMenuAction.edit(enabled: false, onEdit: () {}),
      ],
    ),
  );
}

@widgetbook.UseCase(
  name: 'Many paginated actions',
  type: ChatMessageContextMenu,
)
Widget buildManyActionsContextMenuUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: ChatMessageContextMenu(
      elements: [
        _reactions(),
        for (var index = 1; index <= 21; index++)
          ChatMessageContextMenuAction(
            id: 'action-$index',
            label: 'Action $index',
            iconAsset: ChatMessageContextMenuIcons.info,
            onPressed: () {},
          ),
      ],
    ),
  );
}
