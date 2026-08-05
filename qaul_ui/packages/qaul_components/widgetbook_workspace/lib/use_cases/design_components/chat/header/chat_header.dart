import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../../support/chat_fixtures.dart';
import '../../../../support/widgetbook_preview.dart';

@widgetbook.UseCase(
  name: 'Direct — online',
  type: ChatHeader,
  path: 'design_components/chat/header',
)
Widget buildChatHeaderDirectOnlineUseCase(BuildContext context) {
  return widgetbookTopChromeFrame(
    context,
    ChatHeader(
      onBackPressed: () {},
      avatar: chatFixtureAvatar(initials: 'MJ'),
      displayName: 'Mathias Jud',
      isOnline: true,
      onlineLabel: 'online',
      lastSeenLabel: '',
      extraTopPadding: 32,
      menuEntries: const [
        ChatHeaderMenuEntry(id: 'mute', label: 'Mute'),
        ChatHeaderMenuEntry(id: 'info', label: 'Info'),
      ],
      onMenuSelected: (_) {},
    ),
  );
}

@widgetbook.UseCase(
  name: 'Direct — last seen',
  type: ChatHeader,
  path: 'design_components/chat/header',
)
Widget buildChatHeaderDirectLastSeenUseCase(BuildContext context) {
  return widgetbookTopChromeFrame(
    context,
    ChatHeader(
      onBackPressed: () {},
      avatar: chatFixtureAvatar(initials: 'MJ'),
      displayName: 'Mathias Jud',
      isOnline: false,
      onlineLabel: 'online',
      lastSeenLabel: 'last seen 4 days ago',
      extraTopPadding: 32,
      menuEntries: const [ChatHeaderMenuEntry(id: 'mute', label: 'Mute')],
      onMenuSelected: (_) {},
    ),
  );
}

@widgetbook.UseCase(
  name: 'Group',
  type: ChatHeader,
  path: 'design_components/chat/header',
)
Widget buildChatHeaderGroupUseCase(BuildContext context) {
  return widgetbookTopChromeFrame(
    context,
    ChatHeader.group(
      onBackPressed: () {},
      avatar: chatFixtureAvatar(initials: 'QC'),
      groupName: 'qaul contributors',
      membersCount: 12,
      extraTopPadding: 32,
      formatMembersCount: (n) => '$n members',
      menuEntries: const [
        ChatHeaderMenuEntry(id: 'settings', label: 'Group settings'),
      ],
      onMenuSelected: (_) {},
    ),
  );
}
