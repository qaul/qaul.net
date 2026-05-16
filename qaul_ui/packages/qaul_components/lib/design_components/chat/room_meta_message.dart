import 'package:flutter/material.dart';

import 'chat_message.dart';

class RoomMetaMessage extends ChatMessage {
  const RoomMetaMessage._({super.key, required this.label});

  final String label;

  factory RoomMetaMessage.date({Key? key, required DateTime date}) {
    return RoomMetaMessage._(key: key, label: _formatDateLabel(date));
  }

  @override
  Widget build(BuildContext context) {
    final onSurface = Theme.of(context).colorScheme.onSurface;
    return Center(
      child: Text(
        label,
        style: TextStyle(
          fontSize: 12,
          height: 1.2,
          color: onSurface.withValues(alpha: 0.55),
        ),
      ),
    );
  }
}

const _weekdays = [
  'Monday',
  'Tuesday',
  'Wednesday',
  'Thursday',
  'Friday',
  'Saturday',
  'Sunday',
];

const _months = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
];

String _formatDateLabel(DateTime date) {
  final weekday = _weekdays[date.weekday - 1];
  final month = _months[date.month - 1];
  return '$weekday, $month ${date.day}, ${date.year}';
}
