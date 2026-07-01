import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(name: 'Duplicate username on join', type: DuplicateUsernameMetaMessage)
Widget buildDuplicateUsernameMetaMessageUseCase(BuildContext context) {
  return const ColoredBox(
    color: Colors.black,
    child: Center(
      child: Padding(
        padding: EdgeInsets.symmetric(horizontal: 24, vertical: 16),
        child: DuplicateUsernameMetaMessage(
          preamble: 'Group member ',
          baseName: 'Nickname',
          middle: ' is renamed ',
          disambiguatedName: 'Nickname 3fr',
          actionLabel: 'Edit user names',
        ),
      ),
    ),
  );
}
