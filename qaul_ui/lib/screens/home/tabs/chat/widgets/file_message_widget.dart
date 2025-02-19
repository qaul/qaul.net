part of 'chat.dart';

class FileMessageWidget extends StatelessWidget {
  const FileMessageWidget({
    super.key,
    required this.message,
    this.isDefaultUser = false,
  });

  final types.FileMessage message;
  final bool isDefaultUser;

  bool _isReceivingFile() {
    var isReceiving = false;
    if (message.metadata?.containsKey('messageState') ?? false) {
      final s = MessageState.fromJson(message.metadata!['messageState']);
      isReceiving = s == MessageState.receiving;
    }
    return isReceiving;
  }

  @override
  Widget build(BuildContext context) {
    var style = Theme.of(context).textTheme.bodyLarge!.copyWith(
          color: isDefaultUser ? Colors.white : Colors.black,
          fontSize: 16,
          fontWeight: FontWeight.w400,
        );

    String? description = message.metadata?['description'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (description != null && description.isNotEmpty) ...[
          Padding(
            padding: const EdgeInsets.all(20),
            child: Text(description, style: style),
          ),
        ],
        Container(
          padding: const EdgeInsets.all(20),
          decoration: const BoxDecoration(color: Colors.black12),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Container(
                width: 42,
                height: 42,
                decoration: const BoxDecoration(
                    color: Colors.black12, shape: BoxShape.circle),
                child: _isReceivingFile()
                    ? const Padding(
                        padding: EdgeInsets.all(8.0),
                        child: CircularProgressIndicator(),
                      )
                    : const Icon(Icons.description),
              ),
              const SizedBox(width: 16),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(message.name, style: style),
                    const SizedBox(height: 4),
                    Text(filesize(message.size), style: style),
                  ],
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
