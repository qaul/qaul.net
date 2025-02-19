import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'widgets.dart';

class QaulDialog extends HookConsumerWidget {
  const QaulDialog({
    super.key,
    required this.content,
    required this.button1Label,
    this.title,
    this.onButton1Pressed,
    this.button2Label,
    this.onButton2Pressed,
  });

  final String? title;
  final Widget content;
  final String button1Label;
  final VoidCallback? onButton1Pressed;
  final String? button2Label;
  final VoidCallback? onButton2Pressed;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return AlertDialog(
      title: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          title != null ? Text(title!) : const SizedBox(),
          IconButtonFactory.close(),
        ],
      ),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          content,
          const SizedBox(height: 20),
          Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              QaulButton(label: button1Label, onPressed: onButton1Pressed),
              const SizedBox(width: 12),
              if (button2Label != null)
                QaulButton(
                  label: button2Label!,
                  onPressed: onButton2Pressed,
                ),
            ],
          ),
        ],
      ),
    );
  }
}
