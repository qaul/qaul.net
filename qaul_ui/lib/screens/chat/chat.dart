import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:bubble/bubble.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart'
    show AttachmentButton, Chat, DefaultChatTheme, SendButtonVisibilityMode;
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:qaul_ui/decorators/cron_task_decorator.dart';
import 'package:qaul_ui/widgets/default_back_button.dart';
import 'package:utils/utils.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

part 'custom_input.dart';

typedef OnSendPressed = void Function(String rawText);

class ChatScreen extends HookConsumerWidget {
  const ChatScreen({
    Key? key,
    required this.user,
    required this.otherUser,
    required this.initialMessages,
    required this.userAppBar,
    required this.onSendPressed,
  }) : super(key: key);

  /// The default user
  final User user;

  /// Someone the default user is having a conversation with
  final User otherUser;
  final List<Message> initialMessages;
  final Widget userAppBar;
  final OnSendPressed onSendPressed;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final room = ref.watch(currentOpenChatRoom)!;

    final refreshCurrentRoom = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      worker.getChatRoomMessages(room.conversationId, lastIndex: room.lastMessageIndex ?? 1);
    }, [UniqueKey()]);

    return Scaffold(
      appBar: AppBar(
        leading: const DefaultBackButton(),
        title: userAppBar,
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
            onSendPressed: (message) => onSendPressed(message.text),
            sendButtonVisibilityMode: SendButtonVisibilityMode.always,
            bubbleBuilder: _bubbleBuilder,
            customBottomWidget: _CustomInput(
              sendButtonVisibilityMode: SendButtonVisibilityMode.always,
              onSendPressed: (message) => onSendPressed(message.text),
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

  List<types.TextMessage>? messages(ChatRoom room) {
    return room.messages
        ?.sorted()
        .map((e) => e.toInternalMessage(e.senderId == user.id ? user : otherUser))
        .toList();
  }

  Widget _bubbleBuilder(
    Widget child, {
    required message,
    required nextMessageInGroup,
  }) {
    return Bubble(
      child: child,
      color:
          user.toInternalUser().id != message.author.id || message.type == types.MessageType.image
              ? const Color(0xfff5f5f7)
              : Colors.grey,
      margin: nextMessageInGroup ? const BubbleEdges.symmetric(horizontal: 6) : null,
      nip: nextMessageInGroup
          ? BubbleNip.no
          : user.toInternalUser().id != message.author.id
              ? BubbleNip.leftBottom
              : BubbleNip.rightBottom,
    );
  }
}

extension _MessageListExtension on List<Message>? {
  List<Message> sorted() => [...?(this?..sort())];
}

extension _MessageExtension on Message {
  types.TextMessage toInternalMessage(User author) {
    return types.TextMessage(
      id: messageIdBase58,
      text: content,
      author: author.toInternalUser(),
    );
  }
}

extension _UserExtension on User {
  types.User toInternalUser() => types.User(id: idBase58, firstName: name);
}
