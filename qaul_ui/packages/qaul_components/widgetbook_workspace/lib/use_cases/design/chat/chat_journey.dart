import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

class ChatJourneyBaselineDesignStory {
  const ChatJourneyBaselineDesignStory();
}

class ChatFooterStateDesignStory {
  const ChatFooterStateDesignStory();
}

final _journeyClock = DateTime(2026, 4, 18, 12, 42);

const _me = ChatUser(id: 'me', name: 'Me');
const _peer = ChatUser(id: 'maxx', name: 'MaxX');

Widget _avatar() {
  return const CircleAvatar(
    backgroundColor: Color(0xFFE95420),
    foregroundColor: Colors.white,
    child: Text(
      'M',
      style: TextStyle(
        fontFamily: 'Roboto',
        fontWeight: FontWeight.w400,
        fontSize: 18,
      ),
    ),
  );
}

List<ChatMessage> _journeyMessages() {
  final today = DateTime(
    _journeyClock.year,
    _journeyClock.month,
    _journeyClock.day,
  );
  final earlier = today.subtract(const Duration(days: 1));

  return [
    TextChatMessage(
      id: 'journey-1',
      sender: _me,
      content: 'Hello in 16px 300 font',
      sentAt: earlier.copyWith(hour: 16, minute: 13),
      receivedAt: earlier.copyWith(hour: 16, minute: 13),
      status: MessageStatus.read,
    ),
    TextChatMessage(
      id: 'journey-2',
      sender: _me,
      content:
          'This is a longer message with no own timestamp followed by another message with timestamp',
      sentAt: earlier.copyWith(hour: 16, minute: 13),
      receivedAt: earlier.copyWith(hour: 16, minute: 13),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'journey-3',
      sender: _me,
      content: 'This one is it',
      sentAt: earlier.copyWith(hour: 16, minute: 20),
      receivedAt: earlier.copyWith(hour: 16, minute: 20),
      status: MessageStatus.read,
    ),
    TextChatMessage(
      id: 'journey-4',
      sender: _peer,
      content: 'Chatpartner is answering',
      sentAt: earlier.copyWith(hour: 18, minute: 9),
      receivedAt: earlier.copyWith(hour: 18, minute: 9),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'journey-5',
      sender: _peer,
      content: 'Another answer',
      sentAt: earlier.copyWith(hour: 18, minute: 29),
      receivedAt: earlier.copyWith(hour: 18, minute: 29),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'journey-6',
      sender: _me,
      content: 'Message',
      sentAt: earlier.copyWith(hour: 19, minute: 23),
      receivedAt: earlier.copyWith(hour: 19, minute: 23),
      status: MessageStatus.read,
    ),
    TextChatMessage(
      id: 'journey-7',
      sender: _peer,
      content: 'Longer message from the chatpartner',
      sentAt: earlier.copyWith(hour: 21, minute: 19),
      receivedAt: earlier.copyWith(hour: 21, minute: 19),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'journey-8',
      sender: _peer,
      content: 'followed by one with time',
      sentAt: earlier.copyWith(hour: 21, minute: 39),
      receivedAt: earlier.copyWith(hour: 21, minute: 39),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'journey-9',
      sender: _me,
      content: 'Message with delay',
      sentAt: earlier.copyWith(hour: 22, minute: 14),
      receivedAt: today.copyWith(hour: 12, minute: 14),
      status: MessageStatus.read,
    ),
    TextChatMessage(
      id: 'journey-10',
      sender: _me,
      content: 'Out and delivered',
      sentAt: _journeyClock.subtract(const Duration(minutes: 12)),
      receivedAt: _journeyClock.subtract(const Duration(minutes: 12)),
      status: MessageStatus.read,
    ),
    TextChatMessage(
      id: 'journey-11',
      sender: _me,
      content: 'Out but not delivered yet',
      sentAt: _journeyClock.subtract(const Duration(minutes: 1)),
      receivedAt: _journeyClock.subtract(const Duration(minutes: 1)),
      status: MessageStatus.sent,
    ),
    TextChatMessage(
      id: 'journey-12',
      sender: _peer,
      content: '**Markdown** _preview_ message for bubble spacing context',
      sentAt: _journeyClock.subtract(const Duration(seconds: 20)),
      receivedAt: _journeyClock.subtract(const Duration(seconds: 20)),
      status: MessageStatus.sent,
    ),
  ];
}

Widget _chatHeader() {
  return ChatHeader(
    applyTopSafeArea: false,
    extraTopPadding: 24,
    onBackPressed: () {},
    avatar: _avatar(),
    displayName: 'MaxX',
    isOnline: true,
    onlineLabel: 'online',
    lastSeenLabel: '',
    menuEntries: const [
      ChatHeaderMenuEntry(id: 'mute', label: 'Mute'),
      ChatHeaderMenuEntry(id: 'info', label: 'Info'),
    ],
    onMenuSelected: (_) {},
  );
}

Widget _timeline() {
  return ChatTimeline.direct(
    currentUser: _me,
    messages: _journeyMessages(),
    clock: _journeyClock,
    padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
  );
}

Widget _footer({
  TextEditingController? controller,
  bool menuOpen = false,
}) {
  return ChatFooter(
    placeholder: 'Secure private message',
    controller: controller,
    initialAttachmentMenuOpen: menuOpen,
    onSend: (_) {},
    onVoicePressed: () {},
    onCameraPressed: () {},
    onMoreAttachmentsPressed: () {},
    onAttachmentPressed: () {},
    onEmojiPressed: () {},
    onLocationPressed: () {},
    voiceTooltip: 'Voice message',
    cameraTooltip: 'Photo',
    attachmentsTooltip: 'Attachment',
    emojiTooltip: 'Emoji',
    locationTooltip: 'Location',
    sendTooltip: 'Send',
  );
}

class _ChatJourneyViewport extends StatelessWidget {
  const _ChatJourneyViewport({required this.footer});

  final Widget footer;

  @override
  Widget build(BuildContext context) {
    final sheet = QaulColorSheet(Theme.of(context).brightness);

    return Material(
      color: sheet.surfaceContainer,
      child: SizedBox.expand(
        child: DecoratedBox(
          decoration: BoxDecoration(color: sheet.background),
          child: Column(
            children: [
              _chatHeader(),
              Expanded(
                child: SingleChildScrollView(
                  reverse: true,
                  child: _timeline(),
                ),
              ),
              footer,
            ],
          ),
        ),
      ),
    );
  }
}

class _JourneyFrame extends StatelessWidget {
  const _JourneyFrame({
    required this.title,
    required this.footer,
  });

  final String title;
  final Widget footer;

  @override
  Widget build(BuildContext context) {
    final sheet = QaulColorSheet(Theme.of(context).brightness);

    return SizedBox(
      width: 393,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.only(bottom: 8),
            child: Text(
              title,
              style: Theme.of(context).textTheme.labelLarge?.copyWith(
                color: Theme.of(
                  context,
                ).colorScheme.onSurface.withValues(alpha: 0.62),
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          Expanded(
            child: DecoratedBox(
              decoration: BoxDecoration(
                color: sheet.background,
                border: Border.all(
                  color: Theme.of(context).colorScheme.primary,
                  width: 1,
                ),
              ),
              child: Column(
                children: [
                  _chatHeader(),
                  Expanded(
                    child: SingleChildScrollView(
                      reverse: true,
                      child: _timeline(),
                    ),
                  ),
                  footer,
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

@widgetbook.UseCase(
  name: 'Baseline',
  type: ChatJourneyBaselineDesignStory,
)
Widget buildChatJourneyBaselineUseCase(BuildContext context) {
  return ColoredBox(
    color: QaulColorSheet(Theme.of(context).brightness).surfaceContainer,
    child: SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      padding: const EdgeInsets.all(24),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _JourneyFrame(title: 'Empty footer', footer: _footer()),
          const SizedBox(width: 24),
          _JourneyFrame(
            title: 'Plus menu / pagination',
            footer: _footer(menuOpen: true),
          ),
          const SizedBox(width: 24),
          _JourneyFrame(
            title: 'Long typed text',
            footer: _footer(
              controller: TextEditingController(
                text:
                    'Start writing a message and writing more text than one single line so the input uses the full width of the container.',
              ),
            ),
          ),
        ],
      ),
    ),
  );
}

@widgetbook.UseCase(
  name: 'Empty footer',
  type: ChatFooterStateDesignStory,
)
Widget buildChatJourneyEmptyFooterUseCase(BuildContext context) {
  return _ChatJourneyViewport(footer: _footer());
}

@widgetbook.UseCase(
  name: 'Plus menu / pagination',
  type: ChatFooterStateDesignStory,
)
Widget buildChatJourneyPlusMenuUseCase(BuildContext context) {
  return _ChatJourneyViewport(footer: _footer(menuOpen: true));
}

@widgetbook.UseCase(
  name: 'Long typed text',
  type: ChatFooterStateDesignStory,
)
Widget buildChatJourneyLongTextUseCase(BuildContext context) {
  return _ChatJourneyViewport(
    footer: _footer(
      controller: TextEditingController(
        text:
            'Start writing a message and writing more text than one single line so the input uses the full width of the container.',
      ),
    ),
  );
}
