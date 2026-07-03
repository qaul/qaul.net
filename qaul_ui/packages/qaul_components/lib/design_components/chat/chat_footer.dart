import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import '../../styles/qaul_color_sheet.dart';

const Color _kActionIconColor = Color(0xFF999999);
const double _kInputBorderRadius = 15;

const String _kComponentsPackage = 'qaul_components';
const String _kMicrophoneSvg = 'assets/icons/microphone.svg';
const String _kPhotosSvg = 'assets/icons/photos.svg';
const String _kPlusSvg = 'assets/icons/plus.svg';
const String _kSendSvg = 'assets/icons/send.svg';
const String _kEmoticonSvg = 'assets/icons/emoticon.svg';
const String _kGeolocationSvg = 'assets/icons/geolocation.svg';

const double _kHorizontalPadding = 16;
const double _kTopPadding = 12;
const double _kBottomPadding = 24;
const double _kFieldInputHorizontal = 16;
const double _kFieldInputVertical = 12;
const double _kFieldFontSize = 17;
const double _kIconWidth = 15;
const double _kIconHeight = 20;
const double _kSendAssetWidth = 26.25;
const double _kSendAssetHeight = 22.5;
const double _kPlusCircleSize = 28;
const double _kPlusAssetSize = 24;
const double _kTextActionSpacing = 20;
const double _kAttachmentIconSpacing = 24;
const double _kActionButtonMinSize = 40;
const double _kAttachmentMenuTopSpacing = 12;
const double _kAttachmentMenuItemSize = 54;
const double _kAttachmentMenuItemRadius = 10;
const double _kAttachmentMenuItemSpacing = 12;
const double _kTypedActionRowBottomPadding = 4;
const double _kTypedActionRowRightPadding = 16;
const int _kAttachmentMenuMaxVisibleActions = 6;

const List<BoxShadow> _kFooterShadowsDark = [
  BoxShadow(offset: Offset(0, 0), blurRadius: 5, color: Color(0x66000000)),
  BoxShadow(offset: Offset(0, -10), blurRadius: 7, color: Color(0x99000000)),
];

const List<BoxShadow> _kFooterShadowsLight = [
  BoxShadow(offset: Offset(0, 0), blurRadius: 5, color: Color(0x33000000)),
  BoxShadow(offset: Offset(0, -10), blurRadius: 7, color: Color(0x66000000)),
];

Color _inputFillColor(Brightness brightness, Color shell) {
  if (brightness == Brightness.dark) {
    return const Color(0xFF1C1C1E);
  }
  return shell;
}

Color _attachmentMenuItemColor(Brightness brightness) {
  if (brightness == Brightness.dark) {
    return const Color(0xFF1C1C1E);
  }
  return const Color(0xFFE5E5E5);
}

Color _inputBorderColor(Brightness brightness, bool isFocused) {
  if (brightness == Brightness.dark) {
    return isFocused ? Colors.white : _kActionIconColor;
  }
  return isFocused ? Colors.black : _kActionIconColor;
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

/// Bottom chat composer: pill field, plus menu when empty, send when the user
/// has entered text.
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
    this.onAttachmentPressed,
    this.onEmojiPressed,
    this.onLocationPressed,
    this.applyBottomSafeArea = true,
    this.initialAttachmentMenuOpen = false,
    this.sendTooltip,
    this.voiceTooltip,
    this.cameraTooltip,
    this.attachmentsTooltip,
    this.emojiTooltip,
    this.locationTooltip,
  });

  /// Hint shown when the field is empty (typically localized).
  final String placeholder;

  final TextEditingController? controller;
  final ValueChanged<String>? onSend;
  final VoidCallback? onVoicePressed;
  final VoidCallback? onCameraPressed;
  final VoidCallback? onMoreAttachmentsPressed;
  final VoidCallback? onAttachmentPressed;
  final VoidCallback? onEmojiPressed;
  final VoidCallback? onLocationPressed;
  final bool applyBottomSafeArea;
  final bool initialAttachmentMenuOpen;
  final String? sendTooltip;
  final String? voiceTooltip;
  final String? cameraTooltip;
  final String? attachmentsTooltip;
  final String? emojiTooltip;
  final String? locationTooltip;

  @override
  State<ChatFooter> createState() => _ChatFooterState();
}

class _ChatFooterState extends State<ChatFooter> {
  late final TextEditingController _ownedController;
  late final FocusNode _inputFocusNode;
  final GlobalKey _composerTextFieldKey = GlobalKey();
  late bool _isAttachmentMenuOpen;

  TextEditingController get _effectiveController =>
      widget.controller ?? _ownedController;

  @override
  void initState() {
    super.initState();
    _ownedController = TextEditingController();
    _isAttachmentMenuOpen =
        widget.initialAttachmentMenuOpen &&
        _effectiveController.text.trim().isEmpty;
    _effectiveController.addListener(_handleTextChanged);
    _inputFocusNode = FocusNode()..addListener(_handleFocusChanged);
  }

  @override
  void didUpdateWidget(covariant ChatFooter oldWidget) {
    super.didUpdateWidget(oldWidget);
    final oldController = oldWidget.controller ?? _ownedController;
    final newController = _effectiveController;
    if (oldController != newController) {
      oldController.removeListener(_handleTextChanged);
      newController.addListener(_handleTextChanged);
      if (newController.text.trim().isNotEmpty && _isAttachmentMenuOpen) {
        _isAttachmentMenuOpen = false;
      }
    }
  }

  @override
  void dispose() {
    _effectiveController.removeListener(_handleTextChanged);
    _inputFocusNode
      ..removeListener(_handleFocusChanged)
      ..dispose();
    if (widget.controller == null) {
      _ownedController.dispose();
    }
    super.dispose();
  }

  void _handleTextChanged() {
    if (!_isAttachmentMenuOpen || _effectiveController.text.trim().isEmpty) {
      return;
    }
    setState(() {
      _isAttachmentMenuOpen = false;
    });
  }

  void _handleFocusChanged() {
    setState(() {});
  }

  void _handleSend() {
    final text = _effectiveController.text.trim();
    if (text.isEmpty) return;
    _closeAttachmentMenu();
    widget.onSend?.call(text);
  }

  void _handleMoreAttachmentsPressed() {
    setState(() {
      _isAttachmentMenuOpen = !_isAttachmentMenuOpen;
    });
  }

  void _closeAttachmentMenu() {
    if (!_isAttachmentMenuOpen) return;
    setState(() {
      _isAttachmentMenuOpen = false;
    });
  }

  void _handleAttachmentAction(VoidCallback? callback) {
    _closeAttachmentMenu();
    callback?.call();
  }

  List<_FooterAttachmentAction> _attachmentActions() {
    return [
      _FooterAttachmentAction(
        id: 'audio',
        icon: const _ChatFooterSvgIcon(asset: _kMicrophoneSvg),
        tooltip: widget.voiceTooltip,
        onPressed: () => _handleAttachmentAction(widget.onVoicePressed),
      ),
      _FooterAttachmentAction(
        id: 'photo',
        icon: const _ChatFooterSvgIcon(asset: _kPhotosSvg),
        tooltip: widget.cameraTooltip,
        onPressed: () => _handleAttachmentAction(widget.onCameraPressed),
      ),
      _FooterAttachmentAction(
        id: 'attachment',
        icon: const Icon(Icons.attach_file_rounded, color: _kActionIconColor),
        tooltip: widget.attachmentsTooltip,
        onPressed: () => _handleAttachmentAction(
          widget.onAttachmentPressed ?? widget.onMoreAttachmentsPressed,
        ),
      ),
      _FooterAttachmentAction(
        id: 'emoji',
        icon: const _ChatFooterSvgIcon(
          asset: _kEmoticonSvg,
          width: 24,
          height: 24,
        ),
        tooltip: widget.emojiTooltip,
        onPressed: () => _handleAttachmentAction(widget.onEmojiPressed),
      ),
      _FooterAttachmentAction(
        id: 'location',
        icon: const _ChatFooterSvgIcon(
          asset: _kGeolocationSvg,
          width: 17,
          height: 25,
        ),
        tooltip: widget.locationTooltip,
        onPressed: () => _handleAttachmentAction(widget.onLocationPressed),
      ),
    ].take(_kAttachmentMenuMaxVisibleActions).toList(growable: false);
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
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          _ComposerPill(
            footer: widget,
            theme: theme,
            fillColor: fillColor,
            controller: _effectiveController,
            focusNode: _inputFocusNode,
            textFieldKey: _composerTextFieldKey,
            onSendPressed: _handleSend,
            onMorePressed: _handleMoreAttachmentsPressed,
          ),
          if (_isAttachmentMenuOpen) ...[
            const SizedBox(height: _kAttachmentMenuTopSpacing),
            _AttachmentSubmenu(actions: _attachmentActions(), theme: theme),
          ],
        ],
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
    required this.focusNode,
    required this.textFieldKey,
    required this.onSendPressed,
    required this.onMorePressed,
  });

  final ChatFooter footer;
  final ThemeData theme;
  final Color fillColor;
  final TextEditingController controller;
  final FocusNode focusNode;
  final GlobalKey textFieldKey;
  final VoidCallback onSendPressed;
  final VoidCallback onMorePressed;

  bool _usesFullWidthTypedLayout(BuildContext context, double maxWidth) {
    if (!maxWidth.isFinite) return true;
    final text = controller.text;
    if (text.contains('\n') || text.contains('\r')) return true;

    final inlineActionsWidth =
        _kPlusCircleSize +
        _kTextActionSpacing +
        _kActionButtonMinSize +
        _kHorizontalPadding;
    final inlineTextWidth = maxWidth -
        inlineActionsWidth -
        _kFieldInputHorizontal -
        _kFieldInputHorizontal;
    if (inlineTextWidth <= 0) return true;

    final painter = TextPainter(
      text: TextSpan(text: text, style: _fieldValueStyle(theme)),
      textDirection: Directionality.of(context),
      textScaler: MediaQuery.textScalerOf(context),
    )..layout(maxWidth: inlineTextWidth);

    return painter.computeLineMetrics().length > 1;
  }

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: fillColor,
        borderRadius: BorderRadius.circular(_kInputBorderRadius),
        border: Border.all(
          color: _inputBorderColor(theme.brightness, focusNode.hasFocus),
          width: 1,
        ),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(_kInputBorderRadius),
        child: LayoutBuilder(
          builder: (context, constraints) {
            return ListenableBuilder(
              listenable: controller,
              builder: (context, _) {
                final hasText = controller.text.trim().isNotEmpty;
                if (hasText &&
                    _usesFullWidthTypedLayout(context, constraints.maxWidth)) {
                  return Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      _ComposerTextField(
                        key: textFieldKey,
                        footer: footer,
                        theme: theme,
                        controller: controller,
                        focusNode: focusNode,
                      ),
                      Padding(
                        padding: const EdgeInsets.only(
                          right: _kTypedActionRowRightPadding,
                          bottom: _kTypedActionRowBottomPadding,
                        ),
                        child: Align(
                          alignment: AlignmentDirectional.centerEnd,
                          child: _TextActions(
                            onMore: onMorePressed,
                            onPressed: onSendPressed,
                            attachmentsTooltip: footer.attachmentsTooltip,
                            tooltip: footer.sendTooltip,
                          ),
                        ),
                      ),
                    ],
                  );
                }
                return Row(
                  children: [
                    Expanded(
                      child: _ComposerTextField(
                    key: textFieldKey,
                        footer: footer,
                        theme: theme,
                        controller: controller,
                        focusNode: focusNode,
                      ),
                    ),
                    if (hasText)
                      Padding(
                        padding: const EdgeInsets.only(right: _kHorizontalPadding),
                        child: _TextActions(
                          onMore: onMorePressed,
                          onPressed: onSendPressed,
                          attachmentsTooltip: footer.attachmentsTooltip,
                          tooltip: footer.sendTooltip,
                        ),
                      )
                    else
                      _AttachmentActions(
                        onVoice: footer.onVoicePressed,
                        onCamera: footer.onCameraPressed,
                        onMore: onMorePressed,
                        voiceTooltip: footer.voiceTooltip,
                        cameraTooltip: footer.cameraTooltip,
                        attachmentsTooltip: footer.attachmentsTooltip,
                      ),
                  ],
                );
              },
            );
          },
        ),
      ),
    );
  }
}

class _ComposerTextField extends StatelessWidget {
  const _ComposerTextField({
    super.key,
    required this.footer,
    required this.theme,
    required this.controller,
    required this.focusNode,
  });

  final ChatFooter footer;
  final ThemeData theme;
  final TextEditingController controller;
  final FocusNode focusNode;

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      focusNode: focusNode,
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
    );
  }
}

class _TextActions extends StatelessWidget {
  const _TextActions({
    required this.onMore,
    required this.onPressed,
    this.attachmentsTooltip,
    this.tooltip,
  });

  final VoidCallback? onMore;
  final VoidCallback onPressed;
  final String? attachmentsTooltip;
  final String? tooltip;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        _PlusInCircleButton(onPressed: onMore, tooltip: attachmentsTooltip),
        const SizedBox(width: _kTextActionSpacing),
        _SendButton(onPressed: onPressed, tooltip: tooltip),
      ],
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
        icon: const _ChatFooterSvgIcon(
          asset: _kSendSvg,
          width: _kSendAssetWidth,
          height: _kSendAssetHeight,
        ),
        padding: EdgeInsets.zero,
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
          _PlusInCircleButton(onPressed: onMore, tooltip: attachmentsTooltip),
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

class _FooterAttachmentAction {
  const _FooterAttachmentAction({
    required this.id,
    required this.icon,
    required this.onPressed,
    this.tooltip,
  });

  final String id;
  final Widget icon;
  final VoidCallback onPressed;
  final String? tooltip;
}

class _AttachmentSubmenu extends StatelessWidget {
  const _AttachmentSubmenu({required this.actions, required this.theme});

  final List<_FooterAttachmentAction> actions;
  final ThemeData theme;

  @override
  Widget build(BuildContext context) {
    return Align(
      alignment: AlignmentDirectional.centerStart,
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final action in actions) ...[
              _AttachmentSubmenuButton(
                key: ValueKey(action.id),
                action: action,
                theme: theme,
              ),
              if (action != actions.last)
                const SizedBox(width: _kAttachmentMenuItemSpacing),
            ],
          ],
        ),
      ),
    );
  }
}

class _AttachmentSubmenuButton extends StatelessWidget {
  const _AttachmentSubmenuButton({
    super.key,
    required this.action,
    required this.theme,
  });

  final _FooterAttachmentAction action;
  final ThemeData theme;

  @override
  Widget build(BuildContext context) {
    return _wrapTooltip(
      action.tooltip,
      Material(
        color: _attachmentMenuItemColor(theme.brightness),
        borderRadius: BorderRadius.circular(_kAttachmentMenuItemRadius),
        child: InkWell(
          onTap: action.onPressed,
          borderRadius: BorderRadius.circular(_kAttachmentMenuItemRadius),
          child: SizedBox.square(
            dimension: _kAttachmentMenuItemSize,
            child: Center(
              child: IconTheme.merge(
                data: const IconThemeData(color: _kActionIconColor, size: 28),
                child: action.icon,
              ),
            ),
          ),
        ),
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
      colorFilter: const ColorFilter.mode(_kActionIconColor, BlendMode.srcIn),
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
