import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

Widget _frameHeader(BuildContext context, Widget child) {
  final surface = Theme.of(context).scaffoldBackgroundColor;
  return Material(
    child: ColoredBox(
      color: surface,
      child: Column(
        children: [
          child,
          const Expanded(child: SizedBox.expand()),
        ],
      ),
    ),
  );
}

Widget _demoAvatar({required String initials}) {
  return CircleAvatar(
    backgroundColor: const Color(0xFFD35400),
    foregroundColor: Colors.white,
    child: Text(
      initials,
      style: const TextStyle(
        fontFamily: 'Roboto',
        fontWeight: FontWeight.w400,
        fontSize: 18,
      ),
    ),
  );
}

@widgetbook.UseCase(name: 'Direct — online', type: ChatHeader)
Widget buildChatHeaderDirectOnlineUseCase(BuildContext context) {
  return _frameHeader(
    context,
    ChatHeader(
      onBackPressed: () {},
      avatar: _demoAvatar(initials: 'MJ'),
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

@widgetbook.UseCase(name: 'Direct — last seen', type: ChatHeader)
Widget buildChatHeaderDirectLastSeenUseCase(BuildContext context) {
  return _frameHeader(
    context,
    ChatHeader(
      onBackPressed: () {},
      avatar: _demoAvatar(initials: 'MJ'),
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

@widgetbook.UseCase(name: 'Group', type: ChatHeader)
Widget buildChatHeaderGroupUseCase(BuildContext context) {
  return _frameHeader(
    context,
    ChatHeader.group(
      onBackPressed: () {},
      avatar: _demoAvatar(initials: 'QC'),
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
