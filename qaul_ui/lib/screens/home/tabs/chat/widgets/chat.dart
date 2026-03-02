import 'dart:async';
import 'dart:io';

import 'package:audioplayers/audioplayers.dart';
import 'package:collection/collection.dart';
import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:flutter_chat_ui/flutter_chat_ui.dart'
    show
        Chat,
        DefaultChatTheme,
        EmojiEnlargementBehavior,
        InputOptions,
        SendButtonVisibilityMode,
        TextMessage;
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:image_picker/image_picker.dart';
import 'package:logging/logging.dart';
import 'package:mime/mime.dart';
import 'package:open_filex/open_filex.dart';
import 'package:path/path.dart' hide context, Context;
import 'package:path_provider/path_provider.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:record/record.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:utils/utils.dart';

import '../../../../../../decorators/cron_task_decorator.dart';
import '../../../../../../l10n/app_localizations.dart';
import '../../../../../providers/providers.dart';
import '../../../../../utils.dart';
import '../../../../../widgets/widgets.dart';
import '../../tab.dart';
import 'conditional/conditional.dart';

part 'audio_message_widget.dart';

part 'audio_recording.dart';

part 'custom_input.dart';

part 'file_message_widget.dart';

part 'file_sharing.dart';

part 'group_settings.dart';

part 'image_message_widget.dart';

typedef OnSendPressed = void Function(String rawText);

const _kChatRouteName = '/chat';

Future<void> openChat(
  ChatRoom room, {
  required WidgetRef ref,
  required BuildContext context,
  required User user,
  User? otherUser,
}) async {
  ref.read(currentOpenChatRoom.notifier).state = room;

  bool isMobile =
      MediaQuery.of(context).size.width < Responsiveness.kTabletBreakpoint;
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
              otherUser: otherUser,
            ),
        settings: const RouteSettings(name: _kChatRouteName)),
  );
}

class ChatScreen extends StatefulHookConsumerWidget {
  const ChatScreen(
    this.room,
    this.user, {
    super.key,
    this.otherUser,
  });

  final ChatRoom room;

  /// The default user
  final User user;

  /// Someone the default user is having a conversation with. Leave null if group chat
  final User? otherUser;

  @override
  ConsumerState<ChatScreen> createState() => _ChatScreenState();

  @visibleForTesting
  static String translateGroupEventMessage(
    GroupEventContent message,
    User author, {
    required AppLocalizations l10n,
    ChatRoom? room,
  }) {
    if (message.type == GroupEventContentType.none) {
      return '';
    }

    if (message.type == GroupEventContentType.created) {
      return l10n.groupStateEventCreated;
    } else if (message.type == GroupEventContentType.closed) {
      return l10n.groupStateEventClosed;
    } else {
      String event = '';
      switch (message.type) {
        case GroupEventContentType.invited:
          event = l10n.groupEventInvited(author.name);
          break;
        case GroupEventContentType.inviteAccepted:
          event = l10n.groupEventInviteAccepted(author.name);
          break;
        case GroupEventContentType.joined:
          // Only show "joined" message for users who actually accepted invitations
          // Don't show for users who only had pending invites
          if (room != null) {
            final roomUser = room.members
                .firstWhereOrNull((member) => member.id.equals(author.id));
            if (roomUser?.invitationState == InvitationState.sent) {
              // User only had a pending invite, don't show "joined" message
              event = '';
            } else {
              event = l10n.groupEventJoined(author.name);
            }
          } else {
            event = l10n.groupEventJoined(author.name);
          }
          break;
        case GroupEventContentType.left:
          // Only show "left" message for users who were actually in the group
          // Don't show for users who only had pending invites
          if (room != null) {
            final roomUser = room.members
                .firstWhereOrNull((member) => member.id.equals(author.id));
            if (roomUser?.invitationState == InvitationState.sent) {
              // User only had a pending invite, don't show "left" message
              event = '';
            } else {
              event = l10n.groupEventLeft(author.name);
            }
          } else {
            event = l10n.groupEventLeft(author.name);
          }
          break;
        case GroupEventContentType.removed:
          event = l10n.groupEventRemoved(author.name);
          break;
        case GroupEventContentType.none:
        case GroupEventContentType.created:
        case GroupEventContentType.closed:
          break;
      }

      return event;
    }
  }
}

class _ChatScreenState extends ConsumerState<ChatScreen> {
  ChatRoom get room => widget.room;

  User get user => widget.user;

  User? get otherUser => widget.otherUser;

  final Map<String, String> _overflowMenuOptions = {};

  void _handleClick(String value) {
    switch (value) {
      case 'groupSettings':
        Navigator.push(context, MaterialPageRoute(builder: (_) {
          return _GroupSettingsPage(room);
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
    assert(otherUser != null || room.isGroupChatRoom,
        'Must either be a group chat or contain another user');
    _scheduleUpdateCurrentOpenChat();
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _updateMenuOptionsBasedOnRoomType();
  }

  @override
  void didUpdateWidget(covariant ChatScreen oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.room == room) return;
    _updateMenuOptionsBasedOnRoomType();
    _scheduleUpdateCurrentOpenChat();
  }

  @override
  Widget build(BuildContext context) {
    final room = ref.watch(currentOpenChatRoom);

    if (room == null) {
      return const Scaffold(body: Center(child: LoadingIndicator()));
    }

    final refreshCurrentRoom = useCallback(() async {
      if (!mounted) return;
      final worker = ref.read(qaulWorkerProvider);
      worker.getChatRoomMessages(
        room.conversationId,
        lastIndex: room.lastMessageIndex ?? 1,
      );
    }, [room]);

    final closeChat = useCallback(() {
      if (!mounted) return;
      ref.read(currentOpenChatRoom.notifier).state = null;
      if (_kChatRouteName == ModalRoute.of(context)?.settings.name) {
        Navigator.pop(context);
      }
    }, [room]);

    final sendMessage = useCallback((types.PartialText msg) {
      if (!mounted) return;
      final worker = ref.read(qaulWorkerProvider);
      worker.sendMessage(room.conversationId, msg.text);
    }, [room]);

    final l10n = AppLocalizations.of(context)!;
    return Scaffold(
      resizeToAvoidBottomInset: true,
      appBar: AppBar(
        leading: IconButtonFactory(onPressed: closeChat),
        title: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            (room.isGroupChatRoom)
                ? QaulAvatar.groupSmall()
                : QaulAvatar.small(user: otherUser),
            const SizedBox(width: 12),
            Expanded(
              child: Text(
                otherUser?.name ?? room.name ?? 'Group',
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            ),
          ],
        ),
        titleSpacing: 0,
        actions: [
          if (_overflowMenuOptions.isNotEmpty)
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
        schedule: const Duration(milliseconds: 300),
        child: SafeArea(
          bottom: false,
          child: Chat(
            showUserAvatars: true,
            showUserNames: room.isGroupChatRoom,
            user: user.toInternalUser(),
            messages: messages(room, l10n: l10n) ?? [],
            onSendPressed: sendMessage,
            inputOptions: const InputOptions(
              sendButtonVisibilityMode: SendButtonVisibilityMode.always,
            ),
            avatarBuilder: (id) {
              var user =
                  room.members.firstWhereOrNull((u) => id.id == u.idBase58);
              if (user == null) return const SizedBox();
              return QaulAvatar.small(user: user, badgeEnabled: false);
            },
            emptyState: Center(child: Text(l10n.chatEmptyState)),
            bubbleBuilder: _bubbleBuilder,
            customBottomWidget: _CustomInput(
              isDisabled: room.status != ChatRoomStatus.active,
              disabledMessage: room.status != ChatRoomStatus.inviteAccepted
                  ? null
                  : 'Please wait for the admin to confirm your acceptance to send messages',
              sendButtonVisibilityMode: SendButtonVisibilityMode.editing,
              hintText: room.isGroupChatRoom
                  ? l10n.groupChatMessageHint
                  : l10n.securePrivateMessageHint,
              onSendPressed: sendMessage,
              onAttachmentPressed: (room.messages?.isEmpty ?? true)
                  ? null
                  : ({types.PartialText? text}) async {
                      FilePickerResult? result;
                      try {
                        result = await FilePicker.platform.pickFiles();
                      } catch (e) {
                        debugPrint(e.toString());
                      }

                      if (result != null && result.files.single.path != null) {
                        File file = File(result.files.single.path!);

                        if (!context.mounted) return;
                        showModalBottomSheet(
                          context: context,
                          useSafeArea: true,
                          isScrollControlled: true,
                          builder: (context) {
                            final dialog = _SendFileDialog(
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
                            if (!Platform.isIOS) {
                              return dialog;
                            }

                            final bottomPadding =
                                MediaQuery.of(context).viewInsets.bottom;
                            return SingleChildScrollView(
                              child: Container(
                                padding: EdgeInsets.only(bottom: bottomPadding),
                                child: dialog,
                              ),
                            );
                          },
                        );
                      }
                    },
              onPickImagePressed: !(Platform.isAndroid || Platform.isIOS)
                  ? null
                  : (room.messages?.isEmpty ?? true)
                      ? null
                      : ({types.PartialText? text}) async {
                          final result = await ImagePicker()
                              .pickImage(source: ImageSource.camera);

                          if (result != null) {
                            File file = File(result.path);

                            if (!context.mounted) return;
                            showModalBottomSheet(
                              context: context,
                              useSafeArea: true,
                              isScrollControlled: true,
                              builder: (context) {
                                final dialog = _SendFileDialog(
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
                                if (!Platform.isIOS) {
                                  return dialog;
                                }

                                final bottomPadding =
                                    MediaQuery.of(context).viewInsets.bottom;
                                return SingleChildScrollView(
                                  child: Container(
                                    padding: EdgeInsets.only(
                                      bottom: bottomPadding,
                                    ),
                                    child: dialog,
                                  ),
                                );
                              },
                            );
                          }
                        },
              // the record package is not supported on Linux
              onSendAudioPressed: Platform.isLinux
                  ? null
                  : (room.messages?.isEmpty ?? true)
                      ? null
                      : ({types.PartialText? text}) async {
                          // ignore: use_build_context_synchronously
                          if (!context.mounted) return;
                          showModalBottomSheet(
                            context: context,
                            enableDrag: false,
                            isDismissible: false,
                            builder: (_) {
                              return _RecordAudioDialog(
                                room: room,
                                partialMessage: text?.text,
                                onSendPressed: (file, description) {
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
                        },
            ),
            onMessageTap: (context, message) async {
              if (message is! types.FileMessage || _isReceivingFile(message)) {
                return;
              }
              if (Platform.isIOS || Platform.isAndroid) {
                OpenFilex.open(message.uri);
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
            textMessageBuilder: (message,
                {required int messageWidth, required bool showName}) {
              final msgIdx = room.messages!.indexWhere(
                  (element) => element.messageIdBase58 == message.id);

              var prevMsgWasFromSamePerson = false;
              if (msgIdx > 0) {
                final prevMsg = room.messages![msgIdx - 1];
                prevMsgWasFromSamePerson =
                    prevMsg.content is TextMessageContent &&
                        prevMsg.senderIdBase58 == message.author.id;
              }

              return TextMessage(
                message: message,
                usePreviewData: true,
                hideBackgroundOnEmojiMessages: true,
                showName: showName && !prevMsgWasFromSamePerson,
                emojiEnlargementBehavior: EmojiEnlargementBehavior.multi,
                nameBuilder: (usr) {
                  var user = room.members
                      .firstWhereOrNull((u) => usr.id == u.idBase58);
                  if (user == null) return const SizedBox();
                  final color = colorGenerationStrategy(user.idBase58);
                  return Padding(
                    padding: const EdgeInsets.only(bottom: 4.0),
                    child: Text(
                      user.name,
                      style: TextStyle(
                        fontSize: 11,
                        color: color,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  );
                },
              );
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
            audioMessageBuilder: (message, {required int messageWidth}) {
              return AudioMessageWidget(
                message: message,
                messageWidth: messageWidth,
                isDefaultUser: message.author.id == user.idBase58,
              );
            },
            theme: DefaultChatTheme(
              userAvatarNameColors: [
                colorGenerationStrategy(otherUser?.idBase58 ?? room.idBase58),
              ],
              backgroundColor: Theme.of(context).scaffoldBackgroundColor,
              sentMessageBodyTextStyle: const TextStyle(
                fontSize: 17,
                color: Colors.white,
              ),
              receivedMessageBodyTextStyle: const TextStyle(
                fontSize: 17,
                color: Colors.black,
              ),
            ),
          ),
        ),
      ),
    );
  }

  User _author(Message e) => e.senderId.equals(user.id)
      ? user
      : ref.read(usersProvider).data.firstWhere((usr) => usr.id.equals(e.senderId));

  List<types.Message>? messages(ChatRoom room,
      {required AppLocalizations l10n}) {
    return room.messages
        ?.sorted()
        .map(
            (e) => e.toInternalMessage(_author(e), ref, l10n: l10n, room: room))
        .toList();
  }

  Widget _bubbleBuilder(
    Widget child, {
    required types.Message message,
    required bool nextMessageInGroup,
  }) {
    if (message.type == types.MessageType.custom) {
      return Container(
        alignment: Alignment.center,
        padding: const EdgeInsets.fromLTRB(0, 4, 0, 6),
        margin: const EdgeInsets.symmetric(vertical: 20),
        decoration: BoxDecoration(
            border: Border.all(color: Colors.grey, width: 0.5),
            borderRadius: const BorderRadius.all(Radius.circular(20))),
        child: child,
      );
    }

    const radius = 20.0;

    return Builder(builder: (context) {
      return Bubble(
        elevation: 0,
        nipRadius: 0,
        nipWidth: 0.1,
        nipHeight: radius,
        radius: const Radius.circular(radius),
        padding: EdgeInsets.zero,
        margin: const EdgeInsets.symmetric(horizontal: 4),
        color: user.toInternalUser().id != message.author.id
            ? Colors.grey.shade200
            : Colors.lightBlue.shade700,
        nip: nextMessageInGroup
            ? BubbleNip.no
            : user.toInternalUser().id != message.author.id
                ? BubbleNip.leftBottom
                : BubbleNip.rightBottom,
        child: ClipRRect(borderRadius: BorderRadius.circular(20), child: child),
      );
    });
  }

  bool _isReceivingFile(types.FileMessage message) {
    var isReceiving = false;
    if (message.metadata?.containsKey('messageState') ?? false) {
      final s = MessageState.fromJson(message.metadata!['messageState']);
      isReceiving = s == MessageState.receiving;
    }
    return isReceiving;
  }

  void _updateMenuOptionsBasedOnRoomType() {
    var l10n = AppLocalizations.of(context)!;
    if (room.isGroupChatRoom && _overflowMenuOptions.isEmpty) {
      _overflowMenuOptions.addAll({'groupSettings': l10n.groupSettings});
    }
    if (room.isDirectChat && _overflowMenuOptions.isNotEmpty) {
      _overflowMenuOptions.clear();
    }
  }
}

extension _MessageExtension on Message {
  types.Message toInternalMessage(User author, WidgetRef ref,
      {required AppLocalizations l10n, ChatRoom? room}) {
    var mappedState = status == MessageState.sent
        ? types.Status.sent
        : status == MessageState.confirmedByAll ||
                status == MessageState.confirmed
            ? types.Status.seen
            : null;

    if (content is TextMessageContent) {
      return types.TextMessage(
        id: messageIdBase58,
        text: (content as TextMessageContent).content,
        author: author.toInternalUser(),
        createdAt: receivedAt.millisecondsSinceEpoch,
        status: mappedState,
      );
    } else if (content is GroupEventContent) {
      return types.SystemMessage(
        id: messageIdBase58,
        text: ChatScreen.translateGroupEventMessage(
          content as GroupEventContent,
          author,
          l10n: l10n,
          room: room,
        ),
        createdAt: receivedAt.millisecondsSinceEpoch,
        status: mappedState,
      );
    } else if (content is FileShareContent) {
      var filePath = (content as FileShareContent).filePath(ref);

      String? mimeStr = lookupMimeType(filePath);
      if (mimeStr != null &&
          RegExp('image/.*').hasMatch(mimeStr) &&
          !filePath.endsWith('svg')) {
        return types.ImageMessage(
          id: messageIdBase58,
          author: author.toInternalUser(),
          createdAt: receivedAt.millisecondsSinceEpoch,
          status: mappedState,
          uri: filePath,
          size: (content as FileShareContent).size,
          name: (content as FileShareContent).fileName,
          metadata: {
            'description': (content as FileShareContent).description,
            'messageState': status.toJson(),
          },
        );
      } else if (mimeStr != null && RegExp('audio/.*').hasMatch(mimeStr)) {
        return types.AudioMessage(
          id: messageIdBase58,
          duration: const Duration(seconds: 100),
          author: author.toInternalUser(),
          createdAt: receivedAt.millisecondsSinceEpoch,
          status: mappedState,
          uri: filePath,
          size: (content as FileShareContent).size,
          name: (content as FileShareContent).fileName,
          metadata: {
            'description': (content as FileShareContent).description,
            'messageState': status.toJson(),
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
        status: mappedState,
        metadata: {
          'description': (content as FileShareContent).description,
          'messageState': status.toJson(),
        },
      );
    }

    return types.TextMessage(
      id: messageIdBase58,
      text: 'THIS MESSAGE COULD NOT BE RENDERED. PLEASE CONTACT SUPPORT.',
      author: author.toInternalUser(),
      createdAt: receivedAt.millisecondsSinceEpoch,
      status: mappedState,
    );
  }
}

extension _UserExtension on User {
  types.User toInternalUser() => types.User(id: idBase58, firstName: name);
}
