import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import '../../styles/qaul_color_sheet.dart';

const Color _kActionIconColor = Color(0xFF999999);
const Color _kInputBorderColor = Color(0xFF999999);
const double _kInputBorderRadius = 15;

const String _kComponentsPackage = 'qaul_components';
const String _kMicrophoneSvg = 'assets/icons/microphone.svg';
const String _kPhotosSvg = 'assets/icons/photos.svg';
const String _kPlusSvg = 'assets/icons/plus.svg';

const double _kHorizontalPadding = 16;
const double _kTopPadding = 12;
const double _kBottomPadding = 24;
const double _kFieldInputHorizontal = 16;
const double _kFieldInputVertical = 12;
const double _kFieldFontSize = 17;
const double _kIconWidth = 15;
const double _kIconHeight = 20;
const double _kSendIconSize = 20;
const double _kPlusCircleSize = 28;
const double _kPlusAssetSize = 24;
const double _kAttachmentIconSpacing = 24;

const List<BoxShadow> _kFooterShadowsDark = [
  BoxShadow(
    offset: Offset(0, 0),
    blurRadius: 5,
    color: Color(0x66000000),
  ),
  BoxShadow(
    offset: Offset(0, -10),
    blurRadius: 7,
    color: Color(0x99000000),
  ),
];

const List<BoxShadow> _kFooterShadowsLight = [
  BoxShadow(
    offset: Offset(0, 0),
    blurRadius: 5,
    color: Color(0x33000000),
  ),
  BoxShadow(
    offset: Offset(0, -10),
    blurRadius: 7,
    color: Color(0x66000000),
  ),
];

Color _inputFillColor(Brightness brightness, Color shell) {
  if (brightness == Brightness.dark) {
    return const Color(0xFF1C1C1E);
  }
  return shell;
}

/// Shared typography for the composer field (colors differ for hint vs value).
TextStyle _fieldBaseTextStyle() {
  return const TextStyle(
    fontFamily: 'Roboto',
    fontSize: _kFieldFontSize,
    fontWeight: FontWeight.w400,
    height: 1.25,
  );
}

TextStyle _fieldValueStyle(ThemeData theme) {
  return _fieldBaseTextStyle().copyWith(color: theme.colorScheme.onSurface);
}

TextStyle _fieldHintStyle(ThemeData theme) {
  return _fieldBaseTextStyle().copyWith(
    color: theme.brightness == Brightness.dark
        ? const Color(0xFF8E8E93)
        : const Color(0xFF9E9E9E),
  );
}

Widget _wrapTooltip(String? message, Widget child) {
  if (message == null || message.isEmpty) return child;
  return Tooltip(message: message, child: child);
}

/// Bottom chat composer: pill field, attachment shortcuts when empty, send when
/// the user has entered text.
///
/// Pass [placeholder] from app localizations (e.g. `securePrivateMessageHint`).
class ChatFooter extends StatefulWidget {
  const ChatFooter({
    super.key,
    required this.placeholder,
    this.controller,
    this.onSend,
    this.onVoicePressed,
    this.onCameraPressed,
    this.onMoreAttachmentsPressed,
    this.applyBottomSafeArea = true,
    this.sendTooltip,
    this.voiceTooltip,
    this.cameraTooltip,
    this.attachmentsTooltip,
  });

  /// Hint shown when the field is empty (typically localized).
  final String placeholder;

  final TextEditingController? controller;
  final ValueChanged<String>? onSend;
  final VoidCallback? onVoicePressed;
  final VoidCallback? onCameraPressed;
  final VoidCallback? onMoreAttachmentsPressed;
  final bool applyBottomSafeArea;
  final String? sendTooltip;
  final String? voiceTooltip;
  final String? cameraTooltip;
  final String? attachmentsTooltip;

  @override
  State<ChatFooter> createState() => _ChatFooterState();
}

class _ChatFooterState extends State<ChatFooter> {
  late final TextEditingController _ownedController;

  TextEditingController get _effectiveController =>
      widget.controller ?? _ownedController;

  @override
  void initState() {
    super.initState();
    _ownedController = TextEditingController();
  }

  @override
  void dispose() {
    if (widget.controller == null) {
      _ownedController.dispose();
    }
    super.dispose();
  }

  void _handleSend() {
    final text = _effectiveController.text.trim();
    if (text.isEmpty) return;
    widget.onSend?.call(text);
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final sheet = QaulColorSheet(theme.brightness);
    final shell = sheet.background;
    final fillColor = _inputFillColor(theme.brightness, shell);
    final shadows = theme.brightness == Brightness.dark
        ? _kFooterShadowsDark
        : _kFooterShadowsLight;

    final inner = Padding(
      padding: const EdgeInsets.fromLTRB(
        _kHorizontalPadding,
        _kTopPadding,
        _kHorizontalPadding,
        _kBottomPadding,
      ),
      child: _ComposerPill(
        footer: widget,
        theme: theme,
        fillColor: fillColor,
        controller: _effectiveController,
        onSendPressed: _handleSend,
      ),
    );

    return DecoratedBox(
      decoration: BoxDecoration(color: shell, boxShadow: shadows),
      child: Material(
        color: Colors.transparent,
        child: widget.applyBottomSafeArea
            ? SafeArea(top: false, child: inner)
            : inner,
      ),
    );
  }
}

/// Pill + [TextField]; trailing actions depend on text and listen to [controller]
/// without rebuilding the whole footer.
class _ComposerPill extends StatelessWidget {
  const _ComposerPill({
    required this.footer,
    required this.theme,
    required this.fillColor,
    required this.controller,
    required this.onSendPressed,
  });

  final ChatFooter footer;
  final ThemeData theme;
  final Color fillColor;
  final TextEditingController controller;
  final VoidCallback onSendPressed;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: fillColor,
        borderRadius: BorderRadius.circular(_kInputBorderRadius),
        border: Border.all(color: _kInputBorderColor, width: 1),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(_kInputBorderRadius),
        child: Row(
          children: [
            Expanded(
              child: TextField(
                controller: controller,
                minLines: 1,
                maxLines: 5,
                style: _fieldValueStyle(theme),
                decoration: InputDecoration(
                  isDense: true,
                  hint: SizedBox(
                    width: double.infinity,
                    child: Text(
                      footer.placeholder,
                      maxLines: 1,
                      softWrap: false,
                      overflow: TextOverflow.ellipsis,
                      style: _fieldHintStyle(theme),
                    ),
                  ),
                  border: InputBorder.none,
                  focusedBorder: InputBorder.none,
                  enabledBorder: InputBorder.none,
                  contentPadding: const EdgeInsets.fromLTRB(
                    _kFieldInputHorizontal,
                    _kFieldInputVertical,
                    _kFieldInputHorizontal,
                    _kFieldInputVertical,
                  ),
                ),
                textCapitalization: TextCapitalization.sentences,
                keyboardType: TextInputType.multiline,
                textInputAction: TextInputAction.newline,
              ),
            ),
            ListenableBuilder(
              listenable: controller,
              builder: (context, _) {
                final hasText = controller.text.trim().isNotEmpty;
                if (hasText) {
                  return _SendButton(
                    onPressed: onSendPressed,
                    tooltip: footer.sendTooltip,
                  );
                }
                return _AttachmentActions(
                  onVoice: footer.onVoicePressed,
                  onCamera: footer.onCameraPressed,
                  onMore: footer.onMoreAttachmentsPressed,
                  voiceTooltip: footer.voiceTooltip,
                  cameraTooltip: footer.cameraTooltip,
                  attachmentsTooltip: footer.attachmentsTooltip,
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}

class _SendButton extends StatelessWidget {
  const _SendButton({required this.onPressed, this.tooltip});

  final VoidCallback onPressed;
  final String? tooltip;

  @override
  Widget build(BuildContext context) {
    return _wrapTooltip(
      tooltip,
      IconButton(
        onPressed: onPressed,
        icon: const Icon(
          Icons.send_rounded,
          size: _kSendIconSize,
          color: _kActionIconColor,
        ),
        padding: const EdgeInsets.only(left: 8, right: 16),
        constraints: const BoxConstraints(minWidth: 40, minHeight: 40),
        visualDensity: VisualDensity.compact,
      ),
    );
  }
}

class _AttachmentActions extends StatelessWidget {
  const _AttachmentActions({
    required this.onVoice,
    required this.onCamera,
    required this.onMore,
    this.voiceTooltip,
    this.cameraTooltip,
    this.attachmentsTooltip,
  });

  final VoidCallback? onVoice;
  final VoidCallback? onCamera;
  final VoidCallback? onMore;
  final String? voiceTooltip;
  final String? cameraTooltip;
  final String? attachmentsTooltip;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(right: _kHorizontalPadding),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          _svgIconButton(
            asset: _kMicrophoneSvg,
            onPressed: onVoice,
            tooltip: voiceTooltip,
          ),
          const SizedBox(width: _kAttachmentIconSpacing),
          _svgIconButton(
            asset: _kPhotosSvg,
            onPressed: onCamera,
            tooltip: cameraTooltip,
          ),
          const SizedBox(width: _kAttachmentIconSpacing),
          _PlusInCircleButton(
            onPressed: onMore,
            tooltip: attachmentsTooltip,
          ),
        ],
      ),
    );
  }

  Widget _svgIconButton({
    required String asset,
    required VoidCallback? onPressed,
    String? tooltip,
  }) {
    return _wrapTooltip(
      tooltip,
      IconButton(
        onPressed: onPressed,
        icon: _ChatFooterSvgIcon(asset: asset),
        padding: EdgeInsets.zero,
        constraints: const BoxConstraints(minWidth: 40, minHeight: 40),
        visualDensity: VisualDensity.compact,
      ),
    );
  }
}

class _ChatFooterSvgIcon extends StatelessWidget {
  const _ChatFooterSvgIcon({
    required this.asset,
    this.width = _kIconWidth,
    this.height = _kIconHeight,
  });

  final String asset;
  final double width;
  final double height;

  @override
  Widget build(BuildContext context) {
    return SvgPicture.asset(
      asset,
      package: _kComponentsPackage,
      width: width,
      height: height,
      colorFilter: const ColorFilter.mode(
        _kActionIconColor,
        BlendMode.srcIn,
      ),
    );
  }
}

class _PlusInCircleButton extends StatelessWidget {
  const _PlusInCircleButton({required this.onPressed, this.tooltip});

  final VoidCallback? onPressed;
  final String? tooltip;

  @override
  Widget build(BuildContext context) {
    return _wrapTooltip(
      tooltip,
      Material(
        color: Colors.transparent,
        child: InkWell(
          onTap: onPressed,
          customBorder: const CircleBorder(),
          child: const SizedBox(
            width: _kPlusCircleSize,
            height: _kPlusCircleSize,
            child: Center(
              child: _ChatFooterSvgIcon(
                asset: _kPlusSvg,
                width: _kPlusAssetSize,
                height: _kPlusAssetSize,
              ),
            ),
          ),
        ),
      ),
    );
  }
}
