part of 'chat.dart';

class _SendFileDialog extends HookConsumerWidget {
  const _SendFileDialog(
    this.file, {
    required this.room,
    required this.onSendPressed,
    this.displayName,
    this.initialDescription,
  });
  final File file;
  final ChatRoom room;
  final Function(types.PartialText) onSendPressed;
  final String? displayName;
  final String? initialDescription;

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
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(fileSize(file.lengthSync())),
                  LayoutBuilder(
                    builder: (context, constraints) {
                      final name = displayName ?? basename(file.path);
                      final style = DefaultTextStyle.of(context).style;
                      return Text(
                        shortenFileName(
                          name,
                          maxWidth: constraints.maxWidth,
                          style: style,
                          textScaler: MediaQuery.textScalerOf(context),
                          textDirection: Directionality.of(context),
                          locale: Localizations.maybeLocaleOf(context),
                        ),
                        style: style,
                        maxLines: 1,
                        softWrap: false,
                        semanticsLabel: name,
                      );
                    },
                  ),
                ],
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: IconButtonFactory.close(),
            ),
          ],
        ),
        const SizedBox(height: 8),
        _CustomInput(
          isTextRequired: false,
          initialText: initialDescription,
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
