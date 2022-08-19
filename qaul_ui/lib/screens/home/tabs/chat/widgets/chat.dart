import 'dart:io';
import 'dart:typed_data';

import 'package:better_open_file/better_open_file.dart';
import 'package:bubble/bubble.dart';
import 'package:collection/collection.dart';
import 'package:file_picker/file_picker.dart';
import 'package:filesize/filesize.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart'
    show Chat, DefaultChatTheme, InputOptions, SendButtonVisibilityMode;

import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:image_picker/image_picker.dart';
import 'package:mime/mime.dart';
import 'package:path/path.dart' hide context, Context;
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:utils/utils.dart';

import '../../../../../../decorators/cron_task_decorator.dart';
import '../../../../../decorators/empty_state_text_decorator.dart';
import '../../../../../providers/providers.dart';
import '../../../../../utils.dart';
import '../../../../../widgets/widgets.dart';
import '../current_open_chat_notifier.dart';
import 'conditional/conditional.dart';

part 'custom_input.dart';

part 'file_message_widget.dart';

part 'file_sharing.dart';

part 'image_message_widget.dart';

typedef OnSendPressed = void Function(String rawText);

Future<void> openChat(
  ChatRoom room, {
  required WidgetRef ref,
  required BuildContext context,
  required User user,
  required User otherUser,
}) async {
  ref.read(uiOpenChatProvider.notifier).setCurrent(room);

  bool isMobile = MediaQuery.of(context).size.width < kTabletBreakpoint;
  if (!isMobile) {
    ref.read(homeScreenControllerProvider.notifier).goToTab(TabType.chat);
    return;
  }

  await Navigator.push(
    context,
    MaterialPageRoute(
      builder: (context) => ChatScreen(
        room,
        user,
        otherUser,
      ),
    ),
  );
}

class ChatScreen extends StatefulHookConsumerWidget {
  const ChatScreen(
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

class _ChatScreenState extends ConsumerState<ChatScreen> {
  ChatRoom get room => widget.room;

  User get user => widget.user;

  User get otherUser => widget.otherUser;

  bool isMobile(BuildContext context) =>
      MediaQuery.of(context).size.width < kTabletBreakpoint;

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

  void _scheduleUpdateCurrentOpenChat() =>
      WidgetsBinding.instance.addPostFrameCallback((_) {
        ref.read(currentOpenChatRoom.notifier).state = room;
        ref.read(qaulWorkerProvider).getChatRoomMessages(room.conversationId);
      });

  @override
  void initState() {
    super.initState();
    _scheduleUpdateCurrentOpenChat();
  }

  @override
  void didUpdateWidget(covariant ChatScreen oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.room == room) return;
    _scheduleUpdateCurrentOpenChat();
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
      ref.read(uiOpenChatProvider.notifier).close();
      if (isMobile(context)) Navigator.pop(context);
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
          onPressed: () {
            ref.read(currentOpenChatRoom.notifier).state = null;
            closeChat();
          },
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
            inputOptions: const InputOptions(
              sendButtonVisibilityMode: SendButtonVisibilityMode.always,
            ),
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
              onPickImagePressed: !(Platform.isAndroid || Platform.isIOS)
                  ? null
                  : ({types.PartialText? text}) async {
                      final result = await ImagePicker()
                          .pickImage(source: ImageSource.camera);

                      if (result != null) {
                        File file = File(result.path);

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
              if (Platform.isIOS || Platform.isAndroid) {
                OpenFile.open(message.uri);
                return;
              }

              final file = Uri.file(message.uri);

              final parentDirectory = File.fromUri(file).parent.uri;

              for (final uri in [file, parentDirectory]) {
                if (await canLaunchUrl(uri)) {
                  launchUrl(uri);
                  return;
                }
              }
            },
            fileMessageBuilder: (message, {required int messageWidth}) {
              return SizedBox(
                width: messageWidth.toDouble(),
                child: FileMessageWidget(
                  message: message,
                  isDefaultUser: message.author.id == user.idBase58,
                ),
              );
            },
            imageMessageBuilder: (message, {required int messageWidth}) {
              return ImageMessageWidget(
                message: message,
                messageWidth: messageWidth,
                isDefaultUser: message.author.id == user.idBase58,
              );
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
        color: user.toInternalUser().id != message.author.id
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
          metadata: {
            'description': (content as FileShareContent).description,
          },
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
        metadata: {
          'description': (content as FileShareContent).description,
        },
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
