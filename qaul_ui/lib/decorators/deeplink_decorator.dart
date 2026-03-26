import 'dart:io';

import 'package:app_links/app_links.dart';
import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../helpers/navigation_helper.dart';
import '../providers/providers.dart';
import '../screens/home/tabs/chat/widgets/chat.dart';
import '../stores/stores.dart';

class DeepLinkWrapper extends StatefulHookConsumerWidget {
  const DeepLinkWrapper({
    super.key,
    required this.child,
  });

  final Widget child;

  @override
  ConsumerState<DeepLinkWrapper> createState() => _DeepLinkWrapperState();
}

class _DeepLinkWrapperState extends ConsumerState<DeepLinkWrapper> {
  final _log = Logger('DeepLinkWrapper');

  final links = AppLinks();

  @override
  void initState() {
    super.initState();
    _log.config('platform is ${_isSupported ? '' : 'not'} supported');
    if (_isSupported) _initializeUniLinks();
  }

  bool get _isSupported => Platform.isAndroid || Platform.isIOS;

  @override
  Widget build(BuildContext context) {
    useEffect(() {
      if (!_isSupported) return () {};
      final subscription = links.uriLinkStream.listen(_parseDeepLink);
      return subscription.cancel;
    });

    return widget.child;
  }

  void _initializeUniLinks() async {
    final initialLink = await links.getInitialLink();
    _log.config('initial link: $initialLink');
    if (initialLink != null) _parseDeepLink(initialLink);
  }

  void _parseDeepLink(Uri? link) {
    _log.fine('processing link: $link');
    if (link == null) return;
    if (link.scheme == "qaul") {
      final linkCommand = link.host;
      if (linkCommand == 'public') {
        Navigator.popUntil(context, _reachedHomeScreen);
        ref.read(homeScreenControllerProvider.notifier).goToTab(TabType.public);
      } else if (linkCommand == "chat") {
        final idBase58 = link.path.replaceAll("/", "");
        _navigateToChat(idBase58);
      }

      throw ArgumentError.value(linkCommand, 'DeepLinkWrapper', 'unhandled deeplink command');
    }
  }

  Future<void> _navigateToChat(String id) async {
    final usr = ref.read(defaultUserProvider)!;
    final room = _roomWithId(id);
    if (room == null) return;

    User? otherUser;
    if (!room.isGroupChatRoom) {
      final store = ref.read(usersStoreProvider.notifier);
      // Try the sync store lookup first; if missing, fetch via RPC.
      otherUser = store.otherUserInDirectRoom(room, usr);
      if (otherUser == null) {
        final otherMember = room.members
            .firstWhereOrNull((m) => m.idBase58 != usr.idBase58);
        if (otherMember == null) return;
        otherUser = await store.getByUserID(otherMember.idBase58);
      }
      if (otherUser == null) return;
    }

    if (!mounted) return;
    openChat(room, ref: ref, context: context, user: usr, otherUser: otherUser);
  }

  bool _reachedHomeScreen(Route<dynamic> r) => r.settings.name == NavigationHelper.home;

  ChatRoom? _roomWithId(String id) =>
      ref.read(chatRoomsProvider).firstWhereOrNull((r) => r.idBase58 == id);
}
