import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../support/chat_fixtures.dart';
import '../../../support/widgetbook_preview.dart';

/// Widgetbook node for the full chat screen (header + timeline + footer), as
/// opposed to the single-component stories under `[design components]/chat`.
class ChatJourneyDesignStory {
  const ChatJourneyDesignStory();
}

final _journeyClock = DateTime(2026, 4, 18, 12, 42);

const _kLongDraft =
    'Start writing a message and writing more text than one single line so '
    'the input uses the full width of the container.';

/// Footer states the journey documents. Each is rendered both as its own use
/// case and side by side in [buildChatJourneyBaselineUseCase].
final _footerStates = <({String title, Widget Function() build})>[
  (title: 'Empty footer', build: _emptyFooter),
  (title: 'Plus menu / pagination', build: _plusMenuFooter),
  (title: 'Long typed text', build: _longTextFooter),
];

Widget _emptyFooter() => _footer();

Widget _plusMenuFooter() => _footer(menuOpen: true);

Widget _longTextFooter() =>
    _footer(controller: TextEditingController(text: _kLongDraft));

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

/// Header + scrolled timeline + [footer], on the themed chat canvas.
class _ChatJourneyScreen extends StatelessWidget {
  const _ChatJourneyScreen({required this.footer, this.bordered = false});

  final Widget footer;

  /// Outlines the screen so it reads as a device frame when several are shown
  /// next to each other.
  final bool bordered;

  @override
  Widget build(BuildContext context) {
    return Material(
      color: widgetbookChatSurfaceColor(context),
      child: DecoratedBox(
        decoration: BoxDecoration(
          color: widgetbookChatCanvasColor(context),
          border: bordered
              ? Border.all(color: Theme.of(context).colorScheme.primary)
              : null,
        ),
        child: Column(
          children: [
            ChatHeader(
              applyTopSafeArea: false,
              extraTopPadding: 24,
              onBackPressed: () {},
              avatar: chatFixtureAvatar(initials: 'M'),
              displayName: chatFixturePeer.name,
              isOnline: true,
              onlineLabel: 'online',
              lastSeenLabel: '',
              menuEntries: const [
                ChatHeaderMenuEntry(id: 'mute', label: 'Mute'),
                ChatHeaderMenuEntry(id: 'info', label: 'Info'),
              ],
              onMenuSelected: (_) {},
            ),
            Expanded(
              child: SingleChildScrollView(
                reverse: true,
                child: ChatTimeline.direct(
                  currentUser: chatFixtureCurrentUser,
                  messages: buildDirectChatFixtureMessages(
                    clock: _journeyClock,
                  ),
                  clock: _journeyClock,
                  padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
                ),
              ),
            ),
            footer,
          ],
        ),
      ),
    );
  }
}

/// Phone-width [_ChatJourneyScreen] under a caption, for the baseline row.
class _JourneyFrame extends StatelessWidget {
  const _JourneyFrame({required this.title, required this.footer});

  final String title;
  final Widget footer;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return SizedBox(
      width: 393,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Padding(
            padding: const EdgeInsets.only(bottom: 8),
            child: Text(
              title,
              style: theme.textTheme.labelLarge?.copyWith(
                color: theme.colorScheme.onSurface.withValues(alpha: 0.62),
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          Expanded(child: _ChatJourneyScreen(footer: footer, bordered: true)),
        ],
      ),
    );
  }
}

@widgetbook.UseCase(
  name: 'Baseline',
  type: ChatJourneyDesignStory,
  path: '[design]/chat',
)
Widget buildChatJourneyBaselineUseCase(BuildContext context) {
  return ColoredBox(
    color: widgetbookChatSurfaceColor(context),
    child: SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      padding: const EdgeInsets.all(24),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          for (final (index, state) in _footerStates.indexed) ...[
            if (index > 0) const SizedBox(width: 24),
            _JourneyFrame(title: state.title, footer: state.build()),
          ],
        ],
      ),
    ),
  );
}

@widgetbook.UseCase(
  name: 'Empty footer',
  type: ChatJourneyDesignStory,
  path: '[design]/chat',
)
Widget buildChatJourneyEmptyFooterUseCase(BuildContext context) {
  return _ChatJourneyScreen(footer: _emptyFooter());
}

@widgetbook.UseCase(
  name: 'Plus menu / pagination',
  type: ChatJourneyDesignStory,
  path: '[design]/chat',
)
Widget buildChatJourneyPlusMenuUseCase(BuildContext context) {
  return _ChatJourneyScreen(footer: _plusMenuFooter());
}

@widgetbook.UseCase(
  name: 'Long typed text',
  type: ChatJourneyDesignStory,
  path: '[design]/chat',
)
Widget buildChatJourneyLongTextUseCase(BuildContext context) {
  return _ChatJourneyScreen(footer: _longTextFooter());
}
