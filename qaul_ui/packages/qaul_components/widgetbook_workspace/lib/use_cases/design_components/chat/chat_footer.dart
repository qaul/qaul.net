import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

/// English default matches [app_en.arb] `securePrivateMessageHint` (integrators
/// should pass `AppLocalizations.of(context).securePrivateMessageHint`).
const String _kPlaceholderEn = 'Secure private message';

Widget _lightFooter(Widget child) {
  return Theme(
    data: QaulAppTheme.light,
    child: Builder(builder: (context) => _frameFooter(context, child)),
  );
}

Widget _frameFooter(BuildContext context, Widget child) {
  final sheet = QaulColorSheet(Theme.of(context).brightness);
  return Material(
    child: ColoredBox(
      color: sheet.surfaceContainer,
      child: Column(
        children: [
          const Expanded(child: SizedBox.expand()),
          child,
        ],
      ),
    ),
  );
}

@widgetbook.UseCase(name: 'Light — empty actions', type: ChatFooter)
Widget buildChatFooterLightEmptyClosedUseCase(BuildContext context) {
  return _lightFooter(
    ChatFooter(
      placeholder: _kPlaceholderEn,
      onSend: (_) {},
      onVoicePressed: () {},
      onCameraPressed: () {},
      onMoreAttachmentsPressed: () {},
      onAttachmentPressed: () {},
      onEmojiPressed: () {},
      onLocationPressed: () {},
      voiceTooltip: 'Voice message',
      cameraTooltip: 'Photo',
      attachmentsTooltip: 'More',
      emojiTooltip: 'Emoji',
      locationTooltip: 'Location',
      sendTooltip: 'Send',
    ),
  );
}

@widgetbook.UseCase(name: 'Light — submenu open', type: ChatFooter)
Widget buildChatFooterLightEmptyOpenUseCase(BuildContext context) {
  return _lightFooter(
    ChatFooter(
      placeholder: _kPlaceholderEn,
      initialAttachmentMenuOpen: true,
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
    ),
  );
}

@widgetbook.UseCase(name: 'Light — with text', type: ChatFooter)
Widget buildChatFooterLightWithTextUseCase(BuildContext context) {
  return _lightFooter(
    ChatFooter(
      placeholder: _kPlaceholderEn,
      controller: TextEditingController(text: 'Start writing a message'),
      onSend: (_) {},
      onVoicePressed: () {},
      onCameraPressed: () {},
      onMoreAttachmentsPressed: () {},
      sendTooltip: 'Send',
    ),
  );
}

@widgetbook.UseCase(name: 'Empty — attachment actions', type: ChatFooter)
Widget buildChatFooterEmptyClosedUseCase(BuildContext context) {
  return _frameFooter(
    context,
    ChatFooter(
      placeholder: _kPlaceholderEn,
      onSend: (_) {},
      onVoicePressed: () {},
      onCameraPressed: () {},
      onMoreAttachmentsPressed: () {},
      onAttachmentPressed: () {},
      onEmojiPressed: () {},
      onLocationPressed: () {},
      voiceTooltip: 'Voice message',
      cameraTooltip: 'Photo',
      attachmentsTooltip: 'More',
      emojiTooltip: 'Emoji',
      locationTooltip: 'Location',
      sendTooltip: 'Send',
    ),
  );
}

@widgetbook.UseCase(name: 'Empty — submenu open', type: ChatFooter)
Widget buildChatFooterEmptyOpenUseCase(BuildContext context) {
  return _frameFooter(
    context,
    ChatFooter(
      placeholder: _kPlaceholderEn,
      initialAttachmentMenuOpen: true,
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
    ),
  );
}

@widgetbook.UseCase(name: 'With text — plus and send', type: ChatFooter)
Widget buildChatFooterWithTextUseCase(BuildContext context) {
  return _frameFooter(
    context,
    ChatFooter(
      placeholder: _kPlaceholderEn,
      controller: TextEditingController(
        text: 'Hello — icons are hidden while typing.',
      ),
      onSend: (_) {},
      onVoicePressed: () {},
      onCameraPressed: () {},
      onMoreAttachmentsPressed: () {},
      sendTooltip: 'Send',
    ),
  );
}

@widgetbook.UseCase(name: 'Long draft (multiline)', type: ChatFooter)
Widget buildChatFooterLongDraftUseCase(BuildContext context) {
  return _frameFooter(
    context,
    ChatFooter(
      placeholder: _kPlaceholderEn,
      controller: TextEditingController(
        text:
            'Start writing a message and writing more text than one single '
            'line so the input can use the full composer width before the '
            'plus and send actions appear on the lower row.',
      ),
      onSend: (_) {},
      onMoreAttachmentsPressed: () {},
      sendTooltip: 'Send',
    ),
  );
}
