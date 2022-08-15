part of 'chat.dart';

class _SendFileDialog extends HookConsumerWidget {
  const _SendFileDialog(
    this.file, {
    Key? key,
    required this.room,
    required this.onSendPressed,
    this.partialMessage,
  }) : super(key: key);
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

class _FileHistoryPage extends HookConsumerWidget {
  const _FileHistoryPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final files = ref.watch(fileHistoryEntitiesProvider);

    final refreshFilesHistory = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      worker.getFileHistory();
    }, []);

    return CronTaskDecorator(
      callback: () => refreshFilesHistory(),
      schedule: const Duration(milliseconds: 500),
      child: EmptyStateTextDecorator(
        AppLocalizations.of(context)!.noneAvailableMessage,
        isEmpty: files.isEmpty,
        child: Scaffold(
          appBar: AppBar(leading: const IconButtonFactory()),
          body: ListView.builder(
            itemCount: files.length,
            itemBuilder: (context, index) {
              final file = files[index];

              return ListTile(
                leading: const Icon(Icons.insert_drive_file_outlined),
                title: Text(file.name),
                subtitle: Text(file.description),
              );
            },
          ),
        ),
      ),
    );
  }
}
