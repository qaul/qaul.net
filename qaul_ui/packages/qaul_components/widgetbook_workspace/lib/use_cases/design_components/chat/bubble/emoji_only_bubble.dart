import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook/widgetbook.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

import '../../../../support/emoji_catalog.dart';
import '../../../../support/widgetbook_preview.dart';

final _clock = DateTime(2026, 4, 12, 14, 30);

const _allGroups = 'All';

@widgetbook.UseCase(
  name: 'Emoji-only — layouts',
  type: QaulChatBubble,
  path: 'design_components/chat/bubble',
)
Widget buildEmojiOnlyLayoutsUseCase(BuildContext context) {
  final labelStyle = widgetbookMetaLabelStyle(context);

  Widget section(String title, List<Widget> children) => Padding(
    padding: const EdgeInsets.only(bottom: 24),
    child: Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Text(title, style: labelStyle.copyWith(fontWeight: FontWeight.w600)),
        const SizedBox(height: 8),
        ...children,
      ],
    ),
  );

  List<Widget> bothSides(
    String content, {
    String? senderDisplayName,
    ChatReplyPreviewData? replyPreview,
  }) => [
    for (final isOutgoing in [false, true])
      for (final showTimestamp in [true, false])
        _LabelledBubble(
          label:
              '${isOutgoing ? 'Outgoing' : 'Incoming'} · '
              '${showTimestamp ? 'with' : 'without'} timestamp',
          isOutgoing: isOutgoing,
          child: _emojiBubble(
            content,
            isOutgoing: isOutgoing,
            showTimestamp: showTimestamp,
            senderDisplayName: isOutgoing ? null : senderDisplayName,
            replyPreview: replyPreview,
          ),
        ),
  ];

  return SizedBox.expand(
    child: ColoredBox(
      color: widgetbookChatCanvasColor(context),
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            section('One emoji', bothSides('🐰')),
            section('Two emoji', bothSides('🐭🐰')),
            section(
              'Group chat — sender name',
              bothSides('🎉', senderDisplayName: 'Group Member'),
            ),
            section(
              'Reply',
              bothSides(
                '👍🏽',
                replyPreview: const ChatReplyPreviewData(
                  author: 'Group Member',
                  content: 'Shall we meet at the market at noon?',
                ),
              ),
            ),
          ],
        ),
      ),
    ),
  );
}

@widgetbook.UseCase(
  name: 'Emoji-only — catalog',
  type: QaulChatBubble,
  path: 'design_components/chat/bubble',
)
Widget buildEmojiOnlyCatalogUseCase(BuildContext context) {
  final group = context.knobs.object.dropdown<String>(
    label: 'Group',
    options: [_allGroups, ...kEmojiCatalog.keys],
  );
  final isOutgoing = context.knobs.boolean(label: 'Outgoing');
  final showTimestamp = context.knobs.boolean(
    label: 'Show timestamp',
    initialValue: true,
  );
  final emojiPerBubble = context.knobs.object.dropdown<int>(
    label: 'Emoji per bubble',
    options: const [1, 2],
  );

  final entries = _catalogEntries(group);
  final bubbleCount = (entries.length / emojiPerBubble).ceil();

  Widget bubbleAt(int index) {
    final slice = entries
        .skip(index * emojiPerBubble)
        .take(emojiPerBubble)
        .toList(growable: false);
    return Tooltip(
      message: slice.map((e) => '${e.$2} (Emoji ${e.$3})').join('\n'),
      child: Align(
        alignment: isOutgoing ? Alignment.centerRight : Alignment.centerLeft,
        child: _emojiBubble(
          slice.map((e) => e.$1).join(),
          isOutgoing: isOutgoing,
          showTimestamp: showTimestamp,
        ),
      ),
    );
  }

  // Rows size to their tallest bubble, and cells are wide enough that two
  // emoji never wrap (the chat itself gives bubbles at least 272px).
  final cellWidth = emojiPerBubble == 1 ? 112.0 : 176.0;
  return SizedBox.expand(
    child: ColoredBox(
      color: widgetbookChatCanvasColor(context),
      child: LayoutBuilder(
        builder: (context, constraints) {
          const padding = 16.0;
          final columns = ((constraints.maxWidth - 2 * padding) ~/ cellWidth)
              .clamp(1, 1 << 16);
          final rowCount = (bubbleCount / columns).ceil();
          return ListView.builder(
            padding: const EdgeInsets.all(padding),
            itemCount: rowCount,
            itemBuilder: (context, row) => Padding(
              padding: const EdgeInsets.only(bottom: 8),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  for (var column = 0; column < columns; column++)
                    SizedBox(
                      width: cellWidth,
                      child: row * columns + column < bubbleCount
                          ? bubbleAt(row * columns + column)
                          : null,
                    ),
                ],
              ),
            ),
          );
        },
      ),
    ),
  );
}

@widgetbook.UseCase(
  name: 'Bundled emoji font',
  type: EmojiFontCatalog,
  path: 'styles',
)
Widget buildEmojiFontCatalogUseCase(BuildContext context) {
  final group = context.knobs.object.dropdown<String>(
    label: 'Group',
    options: [_allGroups, ...kEmojiCatalog.keys],
  );
  final fontSize = context.knobs.object.dropdown<double>(
    label: 'Font size',
    options: const [16, 32, 48, 64],
    initialOption: 32,
  );
  final useBundledFont = context.knobs.boolean(
    label: 'Use bundled font',
    description: 'Turn off to compare with the platform emoji font.',
    initialValue: true,
  );

  return EmojiFontCatalog(
    group: group,
    fontSize: fontSize,
    useBundledFont: useBundledFont,
  );
}

/// Grid of every catalog emoji rendered with the bundled emoji font, grouped
/// by Unicode category, so the font can be reviewed glyph by glyph.
class EmojiFontCatalog extends StatelessWidget {
  const EmojiFontCatalog({
    super.key,
    this.group = _allGroups,
    this.fontSize = 32,
    this.useBundledFont = true,
  });

  final String group;
  final double fontSize;
  final bool useBundledFont;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final groups = group == _allGroups
        ? kEmojiCatalog.keys.toList(growable: false)
        : [group];
    final emojiStyle = TextStyle(
      fontSize: fontSize,
      height: 1,
      // An empty list (rather than null) overrides the theme-wide fallback.
      fontFamilyFallback: useBundledFont
          ? kQaulEmojiFontFamilyFallback
          : const [],
    );
    final cellExtent = fontSize * 1.6;

    return Material(
      color: widgetbookChatSurfaceColor(context),
      child: CustomScrollView(
        slivers: [
          SliverPadding(
            padding: const EdgeInsets.fromLTRB(16, 16, 16, 0),
            sliver: SliverToBoxAdapter(
              child: Text(
                '${_catalogEntries(group).length} emoji · '
                'Unicode Emoji $kEmojiCatalogUnicodeVersion · '
                '${useBundledFont ? 'Noto Color Emoji (bundled)' : 'platform font'}',
                style: widgetbookMetaLabelStyle(context),
              ),
            ),
          ),
          for (final name in groups) ...[
            SliverPadding(
              padding: const EdgeInsets.fromLTRB(16, 24, 16, 8),
              sliver: SliverToBoxAdapter(
                child: Text(
                  '$name (${kEmojiCatalog[name]!.length})',
                  style: theme.textTheme.titleSmall,
                ),
              ),
            ),
            SliverPadding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              sliver: SliverGrid.builder(
                gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
                  maxCrossAxisExtent: cellExtent,
                ),
                itemCount: kEmojiCatalog[name]!.length,
                itemBuilder: (context, index) {
                  final (emoji, label, since) = kEmojiCatalog[name]![index];
                  return Tooltip(
                    message: '$label (Emoji $since)',
                    child: Center(child: Text(emoji, style: emojiStyle)),
                  );
                },
              ),
            ),
          ],
          const SliverPadding(padding: EdgeInsets.only(bottom: 24)),
        ],
      ),
    );
  }
}

List<EmojiCatalogEntry> _catalogEntries(String group) => group == _allGroups
    ? kEmojiCatalog.values.expand((entries) => entries).toList(growable: false)
    : kEmojiCatalog[group]!;

QaulChatBubble _emojiBubble(
  String content, {
  required bool isOutgoing,
  required bool showTimestamp,
  String? senderDisplayName,
  ChatReplyPreviewData? replyPreview,
}) {
  return QaulChatBubble(
    message: QaulChatBubbleMessage(
      content: content,
      sentAt: _clock.subtract(const Duration(minutes: 5)),
      receivedAt: _clock.subtract(const Duration(minutes: 5)),
      status: MessageStatus.read,
      messageType: isOutgoing ? MessageType.primary : MessageType.secondary,
      edges: [isOutgoing ? TailEdge.bottomEnd : TailEdge.bottomStart],
      senderIdBase58: isOutgoing ? 'me' : 'group-member',
      senderDisplayName: senderDisplayName,
      replyPreview: replyPreview,
    ),
    clock: _clock,
    showTimestamp: showTimestamp,
  );
}

class _LabelledBubble extends StatelessWidget {
  const _LabelledBubble({
    required this.label,
    required this.isOutgoing,
    required this.child,
  });

  final String label;
  final bool isOutgoing;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: Column(
        crossAxisAlignment: isOutgoing
            ? CrossAxisAlignment.end
            : CrossAxisAlignment.start,
        children: [
          Text(label, style: widgetbookMetaLabelStyle(context)),
          const SizedBox(height: 4),
          child,
        ],
      ),
    );
  }
}
