part of 'chat.dart';

class FileMessageWidget extends StatelessWidget {
  const FileMessageWidget({
    Key? key,
    required this.message,
    this.isDefaultUser = false,
  }) : super(key: key);

  final types.FileMessage message;
  final bool isDefaultUser;

  @override
  Widget build(BuildContext context) {
    var style = Theme.of(context).textTheme.bodyText1!.copyWith(
      color: isDefaultUser ? Colors.white : Colors.black,
      fontSize: 16,
    );

    String? description = message.metadata?['description'];
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (description != null && description.isNotEmpty) ...[
            Text(description, style: style),
            const SizedBox(height: 20),
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
                  child: const Icon(Icons.description),
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
      ),
    );
  }
}
