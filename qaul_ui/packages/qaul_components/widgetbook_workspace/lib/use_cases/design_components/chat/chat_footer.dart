import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

/// English default matches [app_en.arb] `securePrivateMessageHint` (integrators
/// should pass `AppLocalizations.of(context).securePrivateMessageHint`).
const String _kPlaceholderEn = 'Secure private message';

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

@widgetbook.UseCase(name: 'Empty — attachment actions', type: ChatFooter)
Widget buildChatFooterEmptyUseCase(BuildContext context) {
  return _frameFooter(
    context,
    ChatFooter(
      placeholder: _kPlaceholderEn,
      onSend: (_) {},
      onVoicePressed: () {},
      onCameraPressed: () {},
      onMoreAttachmentsPressed: () {},
      voiceTooltip: 'Voice message',
      cameraTooltip: 'Photo',
      attachmentsTooltip: 'More',
      sendTooltip: 'Send',
    ),
  );
}

@widgetbook.UseCase(name: 'With text — send action', type: ChatFooter)
Widget buildChatFooterWithTextUseCase(BuildContext context) {
  return _frameFooter(
    context,
    ChatFooter(
      placeholder: _kPlaceholderEn,
      controller: TextEditingController(text: 'Hello — icons are hidden while typing.'),
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
            'This is a longer draft to show how the pill grows vertically '
            'before scrolling inside the field.',
      ),
      onSend: (_) {},
      sendTooltip: 'Send',
    ),
  );
}
