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
const double _kAttachmentIconSpacing = 24;

List<BoxShadow> _chatFooterShadows(Brightness brightness) {
  if (brightness == Brightness.dark) {
    return const [
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
  }
  return const [
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
}

Color _inputFillColor(Brightness brightness, Color shell) {
  if (brightness == Brightness.dark) {
    return const Color(0xFF1C1C1E);
  }
  return shell;
}

TextStyle _placeholderStyle(ThemeData theme) {
  return TextStyle(
    fontFamily: 'Roboto',
    fontSize: _kFieldFontSize,
    fontWeight: FontWeight.w400,
    height: 1.25,
    color: theme.brightness == Brightness.dark
        ? const Color(0xFF8E8E93)
        : const Color(0xFF9E9E9E),
  );
}

TextStyle _inputStyle(ThemeData theme) {
  return TextStyle(
    fontFamily: 'Roboto',
    fontSize: _kFieldFontSize,
    fontWeight: FontWeight.w400,
    height: 1.25,
    color: theme.colorScheme.onSurface,
  );
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
  TextEditingController? _listened;

  TextEditingController get _effectiveController =>
      widget.controller ?? _ownedController;

  @override
  void initState() {
    super.initState();
    _ownedController = TextEditingController();
    _attachTextListener();
  }

  @override
  void didUpdateWidget(covariant ChatFooter oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.controller != widget.controller) {
      _attachTextListener();
    }
  }

  void _attachTextListener() {
    _listened?.removeListener(_onTextChanged);
    _listened = widget.controller ?? _ownedController;
    _listened!.addListener(_onTextChanged);
  }

  @override
  void dispose() {
    _listened?.removeListener(_onTextChanged);
    if (widget.controller == null) {
      _ownedController.dispose();
    }
    super.dispose();
  }

  void _onTextChanged() => setState(() {});

  bool get _hasText => _effectiveController.text.trim().isNotEmpty;

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

    final field = DecoratedBox(
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
                controller: _effectiveController,
                minLines: 1,
                maxLines: 5,
                style: _inputStyle(theme),
                decoration: InputDecoration(
                  isDense: true,
                  hint: SizedBox(
                    width: double.infinity,
                    child: Text(
                      widget.placeholder,
                      maxLines: 1,
                      softWrap: false,
                      overflow: TextOverflow.ellipsis,
                      style: _placeholderStyle(theme),
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
            if (_hasText)
              _SendButton(
                onPressed: _handleSend,
                tooltip: widget.sendTooltip,
              )
            else
              _AttachmentActions(
                onVoice: widget.onVoicePressed,
                onCamera: widget.onCameraPressed,
                onMore: widget.onMoreAttachmentsPressed,
                voiceTooltip: widget.voiceTooltip,
                cameraTooltip: widget.cameraTooltip,
                attachmentsTooltip: widget.attachmentsTooltip,
              ),
          ],
        ),
      ),
    );

    final inner = Padding(
      padding: const EdgeInsets.fromLTRB(
        _kHorizontalPadding,
        _kTopPadding,
        _kHorizontalPadding,
        _kBottomPadding,
      ),
      child: field,
    );

    final content = DecoratedBox(
      decoration: BoxDecoration(
        color: shell,
        boxShadow: _chatFooterShadows(theme.brightness),
      ),
      child: Material(
        color: Colors.transparent,
        child: widget.applyBottomSafeArea
            ? SafeArea(top: false, child: inner)
            : inner,
      ),
    );

    return content;
  }
}

class _SendButton extends StatelessWidget {
  const _SendButton({required this.onPressed, this.tooltip});

  final VoidCallback onPressed;
  final String? tooltip;

  @override
  Widget build(BuildContext context) {
    final button = IconButton(
      onPressed: onPressed,
      icon: const Icon(
        Icons.send_rounded,
        size: _kSendIconSize,
        color: _kActionIconColor,
      ),
      padding: const EdgeInsets.only(left: 8, right: 16),
      constraints: const BoxConstraints(minWidth: 40, minHeight: 40),
      visualDensity: VisualDensity.compact,
    );
    final t = tooltip;
    if (t == null || t.isEmpty) return button;
    return Tooltip(message: t, child: button);
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
      padding: const EdgeInsets.only(right: 16),
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
    final child = IconButton(
      onPressed: onPressed,
      icon: _ChatFooterSvgIcon(asset: asset),
      padding: EdgeInsets.zero,
      constraints: const BoxConstraints(minWidth: 40, minHeight: 40),
      visualDensity: VisualDensity.compact,
    );
    final t = tooltip;
    if (t == null || t.isEmpty) return child;
    return Tooltip(message: t, child: child);
  }
}

class _ChatFooterSvgIcon extends StatelessWidget {
  const _ChatFooterSvgIcon({required this.asset});

  final String asset;

  @override
  Widget build(BuildContext context) {
    return SvgPicture.asset(
      asset,
      package: _kComponentsPackage,
      width: _kIconWidth,
      height: _kIconHeight,
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
    final inner = Material(
      color: Colors.transparent,
      child: InkWell(
        onTap: onPressed,
        customBorder: const CircleBorder(),
        child: SizedBox(
          width: _kPlusCircleSize,
          height: _kPlusCircleSize,
          child: DecoratedBox(
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              border: Border.all(color: _kActionIconColor, width: 1.2),
            ),
            child: const Center(
              child: _ChatFooterSvgIcon(asset: _kPlusSvg),
            ),
          ),
        ),
      ),
    );
    final t = tooltip;
    if (t == null || t.isEmpty) return inner;
    return Tooltip(message: t, child: inner);
  }
}
