import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import 'group_chat_preview.dart';

final _groupPreviewClock = DateTime(2026, 4, 18, 22, 30);

@widgetbook.UseCase(name: 'Group Chat', type: QaulChatBubble)
Widget buildGroupChatUseCase(BuildContext context) {
  return Container(
    color: Colors.black,
    padding: const EdgeInsets.all(16),
    child: Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 500),
        child: GroupChatPreview(
          clock: _groupPreviewClock,
          surfaceColor: Colors.black,
          dateLabelColor: Colors.white.withValues(alpha: 0.7),
          padding: EdgeInsets.zero,
        ),
      ),
    ),
  );
}

