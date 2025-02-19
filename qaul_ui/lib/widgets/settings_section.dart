part of 'widgets.dart';

class SettingsSection extends StatelessWidget {
  const SettingsSection({
    super.key,
    required this.name,
    required this.icon,
    required this.content,
  });

  final String name;
  final Widget icon;
  final Widget content;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            icon,
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 8.0),
              child: Text(name),
            ),
            const Expanded(child: Divider()),
          ],
        ),
        const SizedBox(height: 12),
        content,
      ],
    );
  }
}
