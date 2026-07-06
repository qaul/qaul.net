import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../support/widgetbook_preview.dart';

@widgetbook.UseCase(name: 'Tappable action', type: DuplicateUsernameMetaMessage)
Widget buildDuplicateUsernameMetaMessageUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: DuplicateUsernameMetaMessage(
        prefix: 'Group member Alice was renamed to',
        emphasizedName: 'Alice 3fr',
        actionLabel: 'Edit usernames',
        onEditPressed: () {},
      ),
    ),
  );
}

@widgetbook.UseCase(name: 'Read-only', type: DuplicateUsernameMetaMessage)
Widget buildDuplicateUsernameMetaMessageReadOnlyUseCase(BuildContext context) {
  return widgetbookChatComponentFrame(
    context,
    child: const Padding(
      padding: EdgeInsets.symmetric(vertical: 8),
      child: DuplicateUsernameMetaMessage(
        prefix: 'Group member Alice was renamed to',
        emphasizedName: 'Alice 3fr',
        actionLabel: 'Edit usernames',
      ),
    ),
  );
}
