import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:bubble/bubble.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart'
    show AttachmentButton, Chat, DefaultChatTheme, SendButtonVisibilityMode;
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:qaul_ui/widgets/default_back_button.dart';
import 'package:utils/utils.dart';

part 'custom_input.dart';

typedef OnSendPressed = void Function(String rawText);

class ChatScreen extends StatefulWidget {
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
  _ChatScreenState createState() => _ChatScreenState();
}

class _ChatScreenState extends State<ChatScreen> {
  final List<types.Message> _messages = [];

  @override
  void initState() {
    super.initState();
    _messages.addAll(
      widget.initialMessages.map((e) => e.toInternalMessage(widget.otherUser)),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: const DefaultBackButton(),
        title: widget.userAppBar,
        titleSpacing: 0,
        actions: [
          IconButton(icon: const Icon(Icons.more_vert), onPressed: () {}),
        ],
      ),
      body: SafeArea(
        bottom: false,
        child: Chat(
          showUserAvatars: true,
          user: widget.user.toInternalUser(),
          messages: _messages,
          onSendPressed: (message) => widget.onSendPressed(message.text),
          sendButtonVisibilityMode: SendButtonVisibilityMode.always,
          bubbleBuilder: _bubbleBuilder,
          customBottomWidget: _CustomInput(
            sendButtonVisibilityMode: SendButtonVisibilityMode.always,
            onSendPressed: (message) => widget.onSendPressed(message.text),
          ),
          theme: DefaultChatTheme(
            userAvatarNameColors: [
              colorGenerationStrategy(widget.otherUser.idBase58),
            ],
            backgroundColor: Theme.of(context).scaffoldBackgroundColor,
          ),
        ),
      ),
    );
  }

  Widget _bubbleBuilder(
    Widget child, {
    required message,
    required nextMessageInGroup,
  }) {
    return Bubble(
      child: child,
      color: widget.user.toInternalUser().id != message.author.id ||
              message.type == types.MessageType.image
          ? const Color(0xfff5f5f7)
          : Colors.grey,
      margin: nextMessageInGroup ? const BubbleEdges.symmetric(horizontal: 6) : null,
      nip: nextMessageInGroup
          ? BubbleNip.no
          : widget.user.toInternalUser().id != message.author.id
              ? BubbleNip.leftBottom
              : BubbleNip.rightBottom,
    );
  }
}

extension MessageExtension on Message {
  types.TextMessage toInternalMessage(User author) {
    return types.TextMessage(
      id: messageIdBase58,
      text: content,
      author: author.toInternalUser(),
    );
  }
}

extension UserExtension on User {
  types.User toInternalUser() {
    return types.User(id: idBase58, firstName: name);
  }
}
