part of 'chat.dart';

class _SendFileDialog extends HookConsumerWidget {
  const _SendFileDialog(
    this.file, {
    required this.room,
    required this.onSendPressed,
    this.partialMessage,
  });
  final File file;
  final ChatRoom room;
  final String? partialMessage;
  final Function(types.PartialText) onSendPressed;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const SizedBox(height: 20),
        Row(
          children: [
            const SizedBox(width: 20),
            const Icon(Icons.insert_drive_file_outlined, size: 40),
            const SizedBox(width: 8),
            Flexible(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(filesize(file.lengthSync())),
                  Text(
                    basename(file.path),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
              ),
            ),
            const Expanded(child: SizedBox()),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: IconButtonFactory.close(),
            ),
          ],
        ),
        const SizedBox(height: 8),
        _CustomInput(
          initialText: partialMessage,
          isTextRequired: false,
          hintText: AppLocalizations.of(context)!.chatEmptyMessageHint,
          onSendPressed: (desc) {
            final worker = ref.read(qaulWorkerProvider);
            worker.sendFile(
                pathName: file.path,
                conversationId: room.conversationId,
                description: desc.text);

            Navigator.pop(context);
          },
          sendButtonVisibilityMode: SendButtonVisibilityMode.always,
        ),
      ],
    );
  }
}
