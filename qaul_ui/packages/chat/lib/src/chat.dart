import 'package:flutter/material.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart'
    show Chat, DefaultChatTheme, SendButtonVisibilityMode;
import 'package:qaul_rpc/qaul_rpc.dart';

import 'models/text_message.dart';

typedef OnSendPressed = TextMessage Function(String rawText);

Future<void> showChat({
  required BuildContext context,
  required List<TextMessage> messages,
  required User user,
  required Color otherUserAvatarColor,
  required Widget userAppBarAvatar,
  required OnSendPressed onSendPressed,
}) {
  return Navigator.push(
    context,
    MaterialPageRoute(
      builder: (context) {
        return _ChatUI(
          initialMessages: messages.map((e) => e.toInternalMessage()).toList(),
          user: user.toInternalUser(),
          avatarColor: otherUserAvatarColor,
          userAppBar: userAppBarAvatar,
          onMessageSent: onSendPressed,
        );
      },
    ),
  );
}

class _ChatUI extends StatefulWidget {
  const _ChatUI({
    Key? key,
    required this.user,
    required this.initialMessages,
    required this.avatarColor,
    required this.userAppBar,
    required this.onMessageSent,
  }) : super(key: key);
  final types.User user;
  final List<types.Message> initialMessages;
  final Color avatarColor;
  final Widget userAppBar;
  final OnSendPressed onMessageSent;

  @override
  _ChatUIState createState() => _ChatUIState();
}

class _ChatUIState extends State<_ChatUI> {
  final List<types.Message> _messages = [];

  void _addMessage(types.Message message) =>
      setState(() => _messages.insert(0, message));

  void _handleSendPressed(types.PartialText message) {
    final textMessage = types.TextMessage.fromPartial(
      author: widget.user,
      id: widget.onMessageSent(message.text).idBase58,
      partialText: message,
      createdAt: DateTime.now().millisecondsSinceEpoch,
    );

    _addMessage(textMessage);
  }

  @override
  void initState() {
    super.initState();
    _messages.addAll(widget.initialMessages);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          tooltip: 'Back',
          icon: const Icon(Icons.arrow_back_ios_rounded),
          onPressed: () => Navigator.pop(context),
        ),
        title: widget.userAppBar,
        leadingWidth: 20,
        actions:  [
          IconButton(icon: const Icon(Icons.more_vert), onPressed: () {}),
        ],
      ),
      body: SafeArea(
        bottom: false,
        child: Chat(
          showUserNames: true,
          showUserAvatars: true,
          user: widget.user,
          messages: _messages,
          onSendPressed: _handleSendPressed,
          sendButtonVisibilityMode: SendButtonVisibilityMode.always,
          theme: DefaultChatTheme(
            sendButtonIcon: const Icon(Icons.send, size: 28),
            userAvatarNameColors: [widget.avatarColor],
            inputBackgroundColor: Colors.white,
            inputTextColor: Colors.black,
            primaryColor: Colors.lightBlue.shade600,
            inputTextDecoration: InputDecoration(
              labelText: 'Your message...',
              floatingLabelBehavior: FloatingLabelBehavior.never,
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(20),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
