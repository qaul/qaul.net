import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../support/widgetbook_preview.dart';

@widgetbook.UseCase(name: 'Default', type: GroupJoinMetaMessage)
Widget buildGroupJoinMetaMessageUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: const Padding(
      padding: EdgeInsets.symmetric(vertical: 8),
      child: GroupJoinMetaMessage(
        userName: 'Alice',
        joinedSuffix: 'has joined the group',
      ),
    ),
  );
}
