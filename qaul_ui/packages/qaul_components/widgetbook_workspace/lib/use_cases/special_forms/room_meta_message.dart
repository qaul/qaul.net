import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../support/widgetbook_preview.dart';

@widgetbook.UseCase(name: 'Date', type: RoomMetaMessage)
Widget buildRoomMetaMessageDateUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: Padding(
      padding: const EdgeInsets.symmetric(vertical: 16),
      child: RoomMetaMessage.date(date: DateTime(2026, 4, 12)),
    ),
  );
}
