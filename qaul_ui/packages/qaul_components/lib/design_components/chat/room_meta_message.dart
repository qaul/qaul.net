import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

import 'chat_message.dart';
import 'chat_meta_text_style.dart';

class RoomMetaMessage extends ChatMessage {
  const RoomMetaMessage._({super.key, this.label, this.date})
    : assert(label != null || date != null);

  final String? label;
  final DateTime? date;

  factory RoomMetaMessage.date({Key? key, required DateTime date}) {
    return RoomMetaMessage._(key: key, date: date);
  }

  @override
  Widget build(BuildContext context) {
    final text = label ?? _formatDateLabel(context, date!);
    return Center(child: Text(text, style: chatMetaTextStyle(context)));
  }
}

String _formatDateLabel(BuildContext context, DateTime date) {
  final locale = Localizations.localeOf(context).toString();
  return DateFormat.yMMMMEEEEd(locale).format(date);
}
