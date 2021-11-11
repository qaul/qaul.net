import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:bubble/bubble.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart'
    show AttachmentButton, Chat, DefaultChatTheme, SendButtonVisibilityMode;
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import 'models/text_message.dart';

part 'custom_input.dart';

typedef OnSendPressed = TextMessage Function(String rawText);

class ChatScreen extends StatefulWidget {
  const ChatScreen({
    Key? key,
    required this.user,
    required this.initialMessages,
    required this.otherUserAvatarColor,
    required this.userAppBar,
    required this.onSendPressed,
  }) : super(key: key);
  final User user;
  final List<TextMessage> initialMessages;
  final Color otherUserAvatarColor;
  final Widget userAppBar;
  final OnSendPressed onSendPressed;

  @override
  _ChatScreenState createState() => _ChatScreenState();
}

class _ChatScreenState extends State<ChatScreen> {
  final List<types.Message> _messages = [];

  void _addMessage(types.Message message) =>
      setState(() => _messages.insert(0, message));

  void _handleSendPressed(types.PartialText message) {
    final textMessage = types.TextMessage.fromPartial(
      author: widget.user.toInternalUser(),
      id: widget.onSendPressed(message.text).idBase58,
      partialText: message,
      createdAt: DateTime.now().millisecondsSinceEpoch,
    );

    _addMessage(textMessage);
  }

  @override
  void initState() {
    super.initState();
    _messages.addAll(widget.initialMessages.map((e) => e.toInternalMessage()));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          tooltip: AppLocalizations.of(context)!.backButtonTooltip,
          icon: const Icon(Icons.arrow_back_ios_rounded),
          onPressed: () => Navigator.pop(context),
        ),
        title: widget.userAppBar,
        titleSpacing: 0,
        actions: [
          IconButton(icon: const Icon(Icons.more_vert), onPressed: () {}),
        ],
      ),
      body: SafeArea(
        bottom: false,
        child: Chat(
          showUserNames: true,
          showUserAvatars: true,
          user: widget.user.toInternalUser(),
          messages: _messages,
          onSendPressed: _handleSendPressed,
          sendButtonVisibilityMode: SendButtonVisibilityMode.always,
          bubbleBuilder: _bubbleBuilder,
          customBottomWidget: _CustomInput(
            sendButtonVisibilityMode: SendButtonVisibilityMode.always,
            onSendPressed: _handleSendPressed,
          ),
          theme: DefaultChatTheme(
            userAvatarNameColors: [widget.otherUserAvatarColor],
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
          : Theme.of(context).colorScheme.primary,
      margin: nextMessageInGroup
          ? const BubbleEdges.symmetric(horizontal: 6)
          : null,
      nip: nextMessageInGroup
          ? BubbleNip.no
          : widget.user.toInternalUser().id != message.author.id
              ? BubbleNip.leftBottom
              : BubbleNip.rightBottom,
    );
  }
}
