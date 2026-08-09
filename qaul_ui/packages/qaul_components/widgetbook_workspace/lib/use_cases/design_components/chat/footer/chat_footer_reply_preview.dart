import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

const _reply = ChatFooterReplyPreviewData(
  author: 'Group member 2',
  content: 'Another answer',
);

Widget _frame(BuildContext context, Widget child) {
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

ChatFooter _footer({
  ChatFooterReplyPreviewData reply = _reply,
  TextEditingController? controller,
}) {
  return ChatFooter(
    placeholder: 'Secure private message',
    replyPreview: reply,
    onCancelReply: () {},
    controller: controller,
    onSend: (_) {},
    onMoreAttachmentsPressed: () {},
    cancelReplyTooltip: 'Cancel reply',
    sendTooltip: 'Send',
  );
}

@widgetbook.UseCase(
  name: 'Empty draft',
  type: ChatFooterReplyPreview,
  path: 'design_components/chat/footer',
)
Widget buildReplyFooterEmptyUseCase(BuildContext context) {
  return _frame(context, _footer());
}

@widgetbook.UseCase(
  name: 'With draft',
  type: ChatFooterReplyPreview,
  path: 'design_components/chat/footer',
)
Widget buildReplyFooterDraftUseCase(BuildContext context) {
  return _frame(
    context,
    _footer(controller: TextEditingController(text: 'Example for a reply')),
  );
}

@widgetbook.UseCase(
  name: 'Long excerpt',
  type: ChatFooterReplyPreview,
  path: 'design_components/chat/footer',
)
Widget buildLongReplyFooterUseCase(BuildContext context) {
  return _frame(
    context,
    _footer(
      reply: const ChatFooterReplyPreviewData(
        author: 'A participant with a very long display name',
        content:
            'This selected message is intentionally long so the preview can '
            'demonstrate truncation without breaking the composer layout.',
      ),
    ),
  );
}
