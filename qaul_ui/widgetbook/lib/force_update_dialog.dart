import 'package:flutter/material.dart';
import 'package:qaul_ui/force_update_overlay.dart' show ForceUpdateDialog;
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(name: 'Default', type: ForceUpdateDialog)
Widget buildCoolButtonUseCase(BuildContext context) {
  return ForceUpdateDialog(
    previous: "2.0.0-beta+17",
    required: "2.0.0-beta+18",
    onLinkPressed: () {},
  );
}
