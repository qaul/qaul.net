import 'dart:io';
import 'dart:typed_data';

import 'package:bubble/bubble.dart';
import 'package:collection/collection.dart';
import 'package:file_picker/file_picker.dart';
import 'package:filesize/filesize.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart'
    show Chat, DefaultChatTheme, SendButtonVisibilityMode;
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:mime/mime.dart';
import 'package:path/path.dart' hide context, Context;
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:utils/utils.dart';

import '../../../../../../decorators/cron_task_decorator.dart';
import '../../../../../decorators/empty_state_text_decorator.dart';
import '../../../../../widgets/widgets.dart';

part 'custom_input.dart';

part 'file_sharing.dart';

typedef OnSendPressed = void Function(String rawText);

Future<void> openChat(
  ChatRoom room, {
  required BuildContext context,
  required User user,
  required User otherUser,
  VoidCallback? onBackButtonPressed,
}) {
  return Navigator.push(
    context,
    MaterialPageRoute(
      builder: (context) => ChatScreen(
        room,
        user,
        otherUser,
        onBackButtonPressed,
      ),
    ),
  );
}

class ChatScreen extends StatefulHookConsumerWidget {
  const ChatScreen(
    this.room,
    this.user,
    this.otherUser,
    this.onBackButtonPressed, {
    Key? key,
  }) : super(key: key);

  final ChatRoom room;

  /// The default user
  final User user;

  /// Someone the default user is having a conversation with
  final User otherUser;

  /// If null, back button will pop the current route.
  final VoidCallback? onBackButtonPressed;

  @override
  _ChatScreenState createState() => _ChatScreenState();
}

class _ChatScreenState extends ConsumerState<ChatScreen> {
  ChatRoom get room => widget.room;

  User get user => widget.user;

  User get otherUser => widget.otherUser;

  Map<String, String> get _overflowMenuOptions => {
        'showFiles': 'Show All Files',
      };

  void _handleClick(String value) {
    switch (value) {
      case 'showFiles':
        Navigator.push(context, MaterialPageRoute(builder: (_) {
          return const _FileHistoryPage();
        }));
        break;
    }
  }

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

    final replyToGroupInvite = useCallback((
      Uint8List groupId, {
      required bool accepted,
    }) {
      final worker = ref.read(qaulWorkerProvider);
      worker.replyToGroupInvite(groupId, accepted: accepted);
    }, []);

    return Scaffold(
      appBar: AppBar(
        leading: IconButtonFactory(
          onPressed: widget.onBackButtonPressed ?? closeChat,
        ),
        title: Row(
          children: [
            UserAvatar.small(badgeEnabled: false, user: otherUser),
            const SizedBox(width: 12),
            Text(otherUser.name),
          ],
        ),
        titleSpacing: 0,
        actions: [
          PopupMenuButton<String>(
            onSelected: _handleClick,
            iconSize: 36,
            splashRadius: 20,
            icon: const Icon(Icons.more_vert),
            itemBuilder: (BuildContext context) {
              return _overflowMenuOptions.keys.map((String key) {
                return PopupMenuItem<String>(
                  value: key,
                  child: Text(_overflowMenuOptions[key]!),
                );
              }).toList();
            },
          ),
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
              onAttachmentPressed: ({types.PartialText? text}) async {
                final result = await FilePicker.platform.pickFiles();

                if (result != null && result.files.single.path != null) {
                  File file = File(result.files.single.path!);

                  showModalBottomSheet(
                    context: context,
                    builder: (_) {
                      return _SendFileDialog(
                        file,
                        room: room,
                        partialMessage: text?.text,
                        onSendPressed: (description) {
                          final worker = ref.read(qaulWorkerProvider);
                          worker.sendFile(
                            pathName: file.path,
                            conversationId: room.conversationId,
                            description: description.text,
                          );
                        },
                      );
                    },
                  );
                }
              },
            ),
            onMessageTap: (context, message) async {
              if (message is! types.FileMessage) return;

              final file = Uri.file(message.uri);
              final parentDirectory = File.fromUri(file).parent.uri;

              for (final uri in [file, parentDirectory]) {
                if (await canLaunchUrl(uri)) {
                  launchUrl(uri);
                  return;
                }
              }
            },
            customMessageBuilder: (message, {required int messageWidth}) {
              final invite = GroupInviteContent.fromJson(message.metadata!);
              if (widget.user.id.equals(invite.adminId)) {
                return Text(
                  'Invite for group "${invite.groupName}" sent',
                  style: const TextStyle(fontStyle: FontStyle.italic),
                );
              }
              return SizedBox(
                width: messageWidth.toDouble(),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'You\'ve been invited to join "${invite.groupName}"!',
                      style: const TextStyle(fontSize: 20),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 12),
                    Text('· Number of members: ${invite.numOfMembers}'),
                    Text('· Created at: ${invite.createdAt}'),
                    const SizedBox(height: 20),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        ElevatedButton(
                          onPressed: () => replyToGroupInvite(invite.groupId,
                              accepted: true),
                          child: const Text('JOIN'),
                        ),
                        const SizedBox(width: 12),
                        ElevatedButton(
                          onPressed: () => replyToGroupInvite(invite.groupId,
                              accepted: false),
                          child: const Text('NO, THANKS'),
                        ),
                      ],
                    ),
                  ],
                ),
              );
            },
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

  List<types.Message>? messages(ChatRoom room) {
    return room.messages
        ?.sorted()
        .map((e) => e.toInternalMessage(_author(e), ref.read))
        .toList();
  }

  Widget _bubbleBuilder(
    Widget child, {
    required types.Message message,
    required bool nextMessageInGroup,
  }) {
    return Builder(builder: (context) {
      return Bubble(
        child: child,
        color: user.toInternalUser().id != message.author.id ||
                message.type == types.MessageType.image
            ? const Color(0xfff5f5f7)
            : Colors.lightBlue.shade700,
        margin: nextMessageInGroup
            ? const BubbleEdges.symmetric(horizontal: 6)
            : null,
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
  types.Message toInternalMessage(User author, Reader read) {
    var mappedStatus = status == MessageStatus.sent
        ? types.Status.sent
        : status == MessageStatus.received
            ? types.Status.delivered
            : null;
    if (content is TextMessageContent) {
      return types.TextMessage(
        id: messageIdBase58,
        text: (content as TextMessageContent).content,
        author: author.toInternalUser(),
        createdAt: receivedAt.millisecondsSinceEpoch,
        status: mappedStatus,
      );
    } else if (content is GroupInviteContent) {
      return types.CustomMessage(
        id: messageIdBase58,
        author: author.toInternalUser(),
        createdAt: receivedAt.millisecondsSinceEpoch,
        status: mappedStatus,
        metadata: (content as GroupInviteContent).toJson(),
      );
    } else if (content is FileShareContent) {
      var filePath = (content as FileShareContent).filePath(read);

      String? mimeStr = lookupMimeType(filePath);
      if (mimeStr != null && RegExp('image/.*').hasMatch(mimeStr)) {
        return types.ImageMessage(
          id: messageIdBase58,
          author: author.toInternalUser(),
          createdAt: receivedAt.millisecondsSinceEpoch,
          status: mappedStatus,
          uri: filePath,
          size: (content as FileShareContent).size,
          name: (content as FileShareContent).fileName,
        );
      }

      return types.FileMessage(
        id: messageIdBase58,
        name: (content as FileShareContent).fileName,
        size: (content as FileShareContent).size,
        uri: filePath,
        author: author.toInternalUser(),
        createdAt: receivedAt.millisecondsSinceEpoch,
        status: mappedStatus,
      );
    }

    return types.TextMessage(
      id: messageIdBase58,
      text: 'THIS MESSAGE COULD NOT BE RENDERED. PLEASE CONTACT SUPPORT.',
      author: author.toInternalUser(),
      createdAt: receivedAt.millisecondsSinceEpoch,
      status: mappedStatus,
    );
  }
}

extension _UserExtension on User {
  types.User toInternalUser() => types.User(id: idBase58, firstName: name);
}
