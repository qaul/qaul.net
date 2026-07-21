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
];

@widgetbook.UseCase(
  name: 'Recipient selection with message',
  type: ForwardRecipientSelector,
  path: '[design]/Message forwarding',
)
Widget buildMessageForwardingFlowUseCase(BuildContext context) =>
    const _MessageForwardingFlow();

class _MessageForwardingFlow extends StatefulWidget {
  const _MessageForwardingFlow();

  @override
  State<_MessageForwardingFlow> createState() => _MessageForwardingFlowState();
}

class _MessageForwardingFlowState extends State<_MessageForwardingFlow> {
  final _messageController = TextEditingController(
    text: 'Example for forwarding',
  );
  String? _selectedRecipientId;

  @override
  void dispose() {
    _messageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return ColoredBox(
      color: theme.scaffoldBackgroundColor,
      child: Column(
        children: [
          Expanded(
            child: ForwardRecipientSelector(
              recipients: _recipients,
              initialSelectedRecipientId: _selectedRecipientId,
              onRecipientSelected: (recipient) {
                setState(() => _selectedRecipientId = recipient.id);
              },
              onSearchChanged: (_) {},
              onSearchFilterSelected: (_) {},
              onCancel: () {},
            ),
          ),
          Divider(height: 1, color: theme.dividerColor),
          SafeArea(
            top: false,
            child: ChatFooter(
              controller: _messageController,
              placeholder: 'Message to forward',
              onSend: (_) {},
              onMoreAttachmentsPressed: () {},
              sendTooltip: 'Forward message',
            ),
          ),
        ],
      ),
    );
  }
}
