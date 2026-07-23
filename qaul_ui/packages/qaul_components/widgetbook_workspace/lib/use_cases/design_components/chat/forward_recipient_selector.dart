import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

const _recipients = [
  ForwardRecipient(
    id: 'public',
    displayName: 'Public',
    kind: ForwardRecipientKind.public,
  ),
  ForwardRecipient(
    id: 'cem',
    displayName: 'Cem Member',
    initials: 'CM',
    kind: ForwardRecipientKind.user,
    avatarColor: Colors.teal,
  ),
  ForwardRecipient(
    id: 'grey',
    displayName: 'Grey 2',
    initials: 'G2',
    kind: ForwardRecipientKind.user,
    avatarColor: Colors.orange,
    isOnline: true,
  ),
  ForwardRecipient(
    id: 'gur',
    displayName: 'Gur Girl',
    initials: 'GG',
    kind: ForwardRecipientKind.user,
    avatarColor: Colors.indigo,
  ),
  ForwardRecipient(
    id: 'francis',
    displayName: 'Francis',
    initials: 'F',
    kind: ForwardRecipientKind.user,
    avatarColor: Colors.amber,
    isOnline: true,
  ),
  ForwardRecipient(
    id: 'anna',
    displayName: 'Anna K',
    initials: 'AK',
    kind: ForwardRecipientKind.user,
    avatarColor: Colors.lime,
  ),
  ForwardRecipient(
    id: 'group-1',
    displayName: 'Group 1',
    kind: ForwardRecipientKind.group,
    avatarColor: Colors.purple,
  ),
  ForwardRecipient(
    id: 'group-2',
    displayName: 'Group 2',
    kind: ForwardRecipientKind.group,
    avatarColor: Colors.purple,
  ),
  ForwardRecipient(
    id: 'friends',
    displayName: 'Friends of qaul',
    kind: ForwardRecipientKind.group,
    avatarColor: Colors.purple,
  ),
];

Widget _selector({bool searchOpen = false, String? selectedId}) {
  return ForwardRecipientSelector(
    recipients: _recipients,
    initialSelectedRecipientId: selectedId,
    initialSearchOpen: searchOpen,
    onRecipientSelected: (_) {},
    onSearchChanged: (_) {},
    onSearchFilterSelected: (_) {},
    onCancel: () {},
  );
}

@widgetbook.UseCase(
  name: 'Recipients — unselected',
  type: ForwardRecipientSelector,
)
Widget buildForwardRecipientSelectorUseCase(BuildContext context) =>
    _selector();

@widgetbook.UseCase(
  name: 'Recipients — selected',
  type: ForwardRecipientSelector,
)
Widget buildForwardRecipientSelectorSelectedUseCase(BuildContext context) =>
    _selector(selectedId: 'francis');

@widgetbook.UseCase(name: 'Search — open', type: ForwardRecipientSelector)
Widget buildForwardRecipientSelectorSearchUseCase(BuildContext context) =>
    _selector(searchOpen: true);
