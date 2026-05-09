import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(name: 'Date', type: RoomMetaMessage)
Widget buildRoomMetaMessageDateUseCase(BuildContext context) {
  return Container(
    color: Colors.black,
    alignment: Alignment.center,
    child: Padding(
      padding: const EdgeInsets.symmetric(vertical: 16),
      child: RoomMetaMessage.date(date: DateTime(2026, 4, 12)),
    ),
  );
}
