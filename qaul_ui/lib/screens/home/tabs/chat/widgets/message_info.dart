part of 'chat.dart';

class _MessageInfoSheet extends StatelessWidget {
  const _MessageInfoSheet({
    required this.message,
    required this.senderName,
    required this.onClosePressed,
  });

  final Message message;
  final String senderName;
  final VoidCallback onClosePressed;

  String _statusLabel() => switch (message.status) {
    MessageState.sending => 'Sending',
    MessageState.sent => 'Sent',
    MessageState.confirmed => 'Confirmed',
    MessageState.confirmedByAll => 'Confirmed by all',
    MessageState.receiving => 'Receiving',
    MessageState.received => 'Received',
  };

  String _formatTimestamp(BuildContext context, DateTime timestamp) {
    final locale = Localizations.localeOf(context).toString();
    return DateFormat.yMMMd(locale).add_jm().format(timestamp.toLocal());
  }

  String _contentType() => switch (message.content) {
    TextMessageContent() => 'Text message',
    FileShareContent() => 'Attachment',
    GroupEventContent() => 'Group event',
    _ => 'Message',
  };

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final size = MediaQuery.sizeOf(context);
    final horizontalPadding = size.width >= 600 ? 62.0 : 32.0;

    return SizedBox(
      key: const ValueKey('message-info-sheet'),
      width: double.infinity,
      height: size.height,
      child: Material(
        color: Colors.black,
        child: SingleChildScrollView(
          child: Padding(
            padding: EdgeInsets.fromLTRB(
              horizontalPadding,
              56,
              horizontalPadding,
              32,
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Expanded(
                      child: Text(
                        'Info',
                        style: theme.textTheme.headlineMedium?.copyWith(
                          color: Colors.white,
                          fontWeight: FontWeight.w400,
                        ),
                      ),
                    ),
                    IconButton(
                      key: const ValueKey('close-message-info'),
                      tooltip: 'Close',
                      onPressed: onClosePressed,
                      icon: const Icon(
                        Icons.close,
                        size: 48,
                        color: Color(0xFF999999),
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 28),
                _MessageInfoRow(label: 'Sender', value: senderName),
                _MessageInfoRow(label: 'Type', value: _contentType()),
                _MessageInfoRow(
                  label: 'Status',
                  value: _statusLabel(),
                  valueKey: const ValueKey('message-info-status'),
                ),
                _MessageInfoRow(
                  label: 'Sent',
                  value: _formatTimestamp(context, message.sentAt),
                ),
                _MessageInfoRow(
                  label: 'Received',
                  value: _formatTimestamp(context, message.receivedAt),
                ),
                _MessageInfoRow(
                  label: 'Message ID',
                  value: message.messageIdBase58,
                  selectable: true,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _MessageInfoRow extends StatelessWidget {
  const _MessageInfoRow({
    required this.label,
    required this.value,
    this.valueKey,
    this.selectable = false,
  });

  final String label;
  final String value;
  final Key? valueKey;
  final bool selectable;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context).textTheme;
    final valueText = Text(
      value,
      key: valueKey,
      textAlign: TextAlign.end,
      style: theme.bodyMedium?.copyWith(color: Colors.white),
    );
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Expanded(
            child: Text(
              label,
              style: theme.bodyMedium?.copyWith(color: Colors.white70),
            ),
          ),
          const SizedBox(width: 24),
          Flexible(
            child: selectable ? SelectionArea(child: valueText) : valueText,
          ),
        ],
      ),
    );
  }
}
