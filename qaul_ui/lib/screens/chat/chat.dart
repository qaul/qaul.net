import 'package:bubble/bubble.dart';
import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart'
    show Chat, DefaultChatTheme, SendButtonVisibilityMode;
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

import '../../decorators/cron_task_decorator.dart';
import '../../widgets/widgets.dart';

part 'custom_input.dart';

typedef OnSendPressed = void Function(String rawText);

Future<void> openChat(
  ChatRoom room, {
  required BuildContext context,
  required User user,
  required User otherUser,
}) {
  return Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => _ChatScreen(room, user, otherUser),
      ));
}

class _ChatScreen extends StatefulHookConsumerWidget {
  const _ChatScreen(
    this.room,
    this.user,
    this.otherUser, {
    Key? key,
  }) : super(key: key);

  final ChatRoom room;

  /// The default user
  final User user;

  /// Someone the default user is having a conversation with
  final User otherUser;

  @override
  _ChatScreenState createState() => _ChatScreenState();
}

class _ChatScreenState extends ConsumerState<_ChatScreen> {
  ChatRoom get room => widget.room;

  User get user => widget.user;

  User get otherUser => widget.otherUser;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(currentOpenChatRoom.notifier).state = room;
      ref.read(qaulWorkerProvider).getChatRoomMessages(room.conversationId);
    });
  }

  @override
  Widget build(BuildContext context) {
    final room = ref.watch(currentOpenChatRoom);

    if (room == null) {
      return const Scaffold(body: Center(child: LoadingIndicator()));
    }

    final refreshCurrentRoom = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      worker.getChatRoomMessages(
        room.conversationId,
        lastIndex: room.lastMessageIndex ?? 1,
      );
    }, [UniqueKey()]);

    final closeChat = useCallback(() {
      ref.read(currentOpenChatRoom.notifier).state = null;
      Navigator.pop(context);
    }, []);

    final sendMessage = useCallback((types.PartialText msg) {
      final worker = ref.read(qaulWorkerProvider);
      worker.sendMessage(room.conversationId, msg.text);
    }, [UniqueKey()]);

    return Scaffold(
      appBar: AppBar(
        leading: DefaultBackButton(onPressed: closeChat),
        title: Row(
          children: [
            UserAvatar.small(badgeEnabled: false, user: otherUser),
            const SizedBox(width: 12),
            Text(otherUser.name),
          ],
        ),
        titleSpacing: 0,
        actions: [
          IconButton(icon: const Icon(Icons.more_vert), onPressed: () {}),
        ],
      ),
      body: CronTaskDecorator(
        callback: () => refreshCurrentRoom(),
        schedule: const Duration(milliseconds: 500),
        child: SafeArea(
          bottom: false,
          child: Chat(
            showUserAvatars: true,
            user: user.toInternalUser(),
            messages: messages(room) ?? [],
            onSendPressed: sendMessage,
            sendButtonVisibilityMode: SendButtonVisibilityMode.always,
            bubbleBuilder: _bubbleBuilder,
            customBottomWidget: _CustomInput(
              sendButtonVisibilityMode: SendButtonVisibilityMode.always,
              onSendPressed: sendMessage,
            ),
            theme: DefaultChatTheme(
              userAvatarNameColors: [
                colorGenerationStrategy(otherUser.idBase58),
              ],
              backgroundColor: Theme.of(context).scaffoldBackgroundColor,
            ),
          ),
        ),
      ),
    );
  }

  User _author(Message e) => e.senderId.equals(user.id) ? user : otherUser;

  List<types.TextMessage>? messages(ChatRoom room) {
    return room.messages?.sorted().map((e) => e.toInternalMessage(_author(e))).toList();
  }

  Widget _bubbleBuilder(
    Widget child, {
    required types.Message message,
    required bool nextMessageInGroup,
  }) {
    return Builder(builder: (context) {
      return Bubble(
        child: child,
        color:
            user.toInternalUser().id != message.author.id || message.type == types.MessageType.image
                ? const Color(0xfff5f5f7)
                : Colors.lightBlue.shade700,
        margin: nextMessageInGroup ? const BubbleEdges.symmetric(horizontal: 6) : null,
        nip: nextMessageInGroup
            ? BubbleNip.no
            : user.toInternalUser().id != message.author.id
                ? BubbleNip.leftBottom
                : BubbleNip.rightBottom,
      );
    });
  }
}

extension _MessageExtension on Message {
  types.TextMessage toInternalMessage(User author) {
    return types.TextMessage(
      id: messageIdBase58,
      text: content,
      author: author.toInternalUser(),
      createdAt: receivedAt.millisecondsSinceEpoch,
    );
  }
}

extension _UserExtension on User {
  types.User toInternalUser() => types.User(id: idBase58, firstName: name);
}
