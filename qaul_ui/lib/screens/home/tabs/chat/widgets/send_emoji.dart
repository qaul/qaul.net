part of 'chat.dart';

class SendEmojiDialog extends StatelessWidget {
  const SendEmojiDialog({
    super.key,
    required this.onEmojiSelected,
  });

  final void Function(String emoji) onEmojiSelected;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 300,
      child: EmojiPicker(
        onEmojiSelected: (category, emoji) {
          onEmojiSelected(emoji.emoji);
          Navigator.pop(context);
        },
        config: Config(
          height: 300,
          checkPlatformCompatibility: true,
          emojiSet: getDefaultEmojiLocale,
          locale: const Locale('en'),
          emojiTextStyle: const TextStyle(fontSize: 18),
          customBackspaceIcon: const Icon(Icons.backspace, color: Colors.blue),
          customSearchIcon: const Icon(Icons.search, color: Colors.blue),
          viewOrderConfig: const ViewOrderConfig(),
          emojiViewConfig: const EmojiViewConfig(),
          skinToneConfig: const SkinToneConfig(),
          categoryViewConfig: const CategoryViewConfig(),
          bottomActionBarConfig: const BottomActionBarConfig(),
          searchViewConfig: const SearchViewConfig(),
        ),
      ),
    );
  }
}
