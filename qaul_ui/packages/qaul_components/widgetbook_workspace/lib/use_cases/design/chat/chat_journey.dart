import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../support/chat_fixtures.dart';

class ChatJourneyBaselineDesignStory {
  const ChatJourneyBaselineDesignStory();
}

class ChatFooterStateDesignStory {
  const ChatFooterStateDesignStory();
}

final _journeyClock = DateTime(2026, 4, 18, 12, 42);

List<ChatMessage> _journeyMessages() {
  return [
    ...buildDirectChatFixtureMessages(clock: _journeyClock),
    TextChatMessage(
      id: 'journey-12',
      sender: chatFixturePeer,
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
    avatar: chatFixtureAvatar(
      initials: 'M',
      backgroundColor: const Color(0xFFE95420),
    ),
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
    currentUser: chatFixtureCurrentUser,
    messages: _journeyMessages(),
    clock: _journeyClock,
    padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
  );
}

Widget _footer({TextEditingController? controller, bool menuOpen = false}) {
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

Widget _chatJourneyBody({required Widget footer}) {
  return Column(
    children: [
      _chatHeader(),
      Expanded(child: SingleChildScrollView(reverse: true, child: _timeline())),
      footer,
    ],
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
          child: _chatJourneyBody(footer: footer),
        ),
      ),
    );
  }
}

class _JourneyFrame extends StatelessWidget {
  const _JourneyFrame({required this.title, required this.footer});

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
              child: _chatJourneyBody(footer: footer),
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
  path: '[design]/chat',
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
  path: '[design]/chat',
)
Widget buildChatJourneyEmptyFooterUseCase(BuildContext context) {
  return _ChatJourneyViewport(footer: _footer());
}

@widgetbook.UseCase(
  name: 'Plus menu / pagination',
  type: ChatFooterStateDesignStory,
  path: '[design]/chat',
)
Widget buildChatJourneyPlusMenuUseCase(BuildContext context) {
  return _ChatJourneyViewport(footer: _footer(menuOpen: true));
}

@widgetbook.UseCase(
  name: 'Long typed text',
  type: ChatFooterStateDesignStory,
  path: '[design]/chat',
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
