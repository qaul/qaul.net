part of 'chat.dart';

class NewLineIntent extends Intent {
  const NewLineIntent();
}

class SendMessageIntent extends Intent {
  const SendMessageIntent();
}

/// The original [Input] class from flutter_chat_ui provided no customization for
/// the spacing of the Send button spacing.
///
/// A hard-coded value made so it was not aligned when the app is in a RTL language.
class _CustomInput extends StatefulWidget {
  /// Creates [_CustomInput] widget
  const _CustomInput({
    required this.onSendPressed,
    required this.sendButtonVisibilityMode,
    required this.hintText,
    this.onAttachmentPressed,
    this.onPickImagePressed,
    this.onSendAudioPressed,
    this.onSendEmojiPicker,
    this.onSendLocationPressed,
    this.initialText,
    this.disabledMessage,
    this.isDisabled = false,
    this.isTextRequired = true,
  });

  final void Function(types.PartialText) onSendPressed;

  final Function({types.PartialText? text})? onAttachmentPressed;

  final Function({types.PartialText? text})? onPickImagePressed;

  final Function({types.PartialText? text})? onSendAudioPressed;

  final Function({types.PartialText? text})? onSendLocationPressed;

  final Function({types.PartialText? text})? onSendEmojiPicker;

  final SendButtonVisibilityMode sendButtonVisibilityMode;

  final String? initialText;

  final bool isDisabled;

  final String? disabledMessage;

  final String hintText;

  final bool isTextRequired;

  @override
  _CustomInputState createState() => _CustomInputState();
}

/// [_CustomInput] widget state
class _CustomInputState extends State<_CustomInput> {
  final _inputFocusNode = FocusNode();
  bool _sendButtonVisible = false;
  late final TextEditingController _textController;

  @override
  void initState() {
    super.initState();

    _textController = TextEditingController(text: widget.initialText);

    if (widget.sendButtonVisibilityMode == SendButtonVisibilityMode.editing) {
      _sendButtonVisible = _textController.text.trim() != '';
      _textController.addListener(_handleTextControllerChange);
    } else {
      _sendButtonVisible = true;
    }
  }

  @override
  void dispose() {
    _inputFocusNode.dispose();
    _textController.dispose();
    super.dispose();
  }

  void _handleSendPressed({types.PartialText? locationMessage}) {
    final trimmedText = _textController.text.trim();
    if (trimmedText != '' || locationMessage != null) {
      final partialText =
          locationMessage ?? types.PartialText(text: trimmedText);
      widget.onSendPressed(partialText);
      _textController.clear();
    }
  }

  void _sendFilePressed(Function({types.PartialText? text})? callback) {
    if (callback == null) return;

    final trimmedText = _textController.text.trim();
    if (trimmedText == '') {
      callback();
      return;
    }

    final partialText = types.PartialText(text: trimmedText);
    callback(text: partialText);
    _textController.clear();
  }

  void _handleTextControllerChange() {
    setState(() {
      _sendButtonVisible = _textController.text.trim() != '';
    });
  }

  @override
  Widget build(BuildContext context) {
    final query = MediaQuery.of(context);

    return Stack(
      alignment: Alignment.center,
      children: [
        Opacity(
          opacity: widget.isDisabled ? 0.3 : 1,
          child: IgnorePointer(
            ignoring: widget.isDisabled,
            child: GestureDetector(
              onTap: () => _inputFocusNode.requestFocus(),
              child: Shortcuts(
                shortcuts: {
                  LogicalKeySet(LogicalKeyboardKey.enter):
                      const SendMessageIntent(),
                  LogicalKeySet(
                          LogicalKeyboardKey.enter, LogicalKeyboardKey.alt):
                      const NewLineIntent(),
                  LogicalKeySet(
                          LogicalKeyboardKey.enter, LogicalKeyboardKey.shift):
                      const NewLineIntent(),
                },
                child: Actions(
                  actions: {
                    SendMessageIntent: CallbackAction<SendMessageIntent>(
                      onInvoke: (SendMessageIntent intent) =>
                          _handleSendPressed(),
                    ),
                    NewLineIntent: CallbackAction<NewLineIntent>(
                      onInvoke: (NewLineIntent intent) {
                        final newValue = '${_textController.text}\r\n';
                        _textController.value = TextEditingValue(
                          text: newValue,
                          selection: TextSelection.fromPosition(
                            TextPosition(offset: newValue.length),
                          ),
                        );
                        return null;
                      },
                    ),
                  },
                  child: Focus(
                    autofocus: true,
                    child: Material(
                      borderRadius: BorderRadius.circular(20),
                      color: Colors.transparent,
                      child: Container(
                        padding: EdgeInsets.fromLTRB(
                          24 + query.padding.left,
                          20,
                          24 + query.padding.right,
                          20 + query.viewInsets.bottom + query.padding.bottom,
                        ),
                        child: Row(
                          children: [
                            Expanded(
                              child: TextField(
                                controller: _textController,
                                style: const TextStyle(fontSize: 17),
                                decoration: InputDecoration(
                                  labelText: widget.hintText,
                                  suffixIcon: Row(
                                    mainAxisSize: MainAxisSize.min,
                                    children: [
                                      if (widget.onSendLocationPressed != null)
                                        _AttachmentButton(
                                          icon: Icons.location_on,
                                          onPressed: () async {
                                            await Navigator.push(
                                              context,
                                              MaterialPageRoute(
                                                builder: (context) => MapScreen(
                                                  key: const ValueKey('map'),
                                                  onLocationSelected:
                                                      (position) {
                                                    final roundedLat = position
                                                        .latitude
                                                        .toStringAsFixed(2);
                                                    final roundedLng = position
                                                        .longitude
                                                        .toStringAsFixed(2);
                                                    final locationMessage =
                                                        types.PartialText(
                                                      text:
                                                          '📍 Location: $roundedLat, $roundedLng',
                                                    );
                                                    _handleSendPressed(
                                                        locationMessage:
                                                            locationMessage);
                                                  },
                                                ),
                                              ),
                                            );
                                          },
                                        ),
                                      if (widget.onAttachmentPressed != null)
                                        _AttachmentButton(
                                          onPressed: () => _sendFilePressed(
                                              widget.onAttachmentPressed),
                                          tooltip: AppLocalizations.of(context)!
                                              .sendFileTooltip,
                                        ),
                                      if (widget.onSendEmojiPicker != null)
                                        _AttachmentButton(
                                          icon: Icons.emoji_emotions,
                                          onPressed: widget.onSendEmojiPicker,
                                          tooltip: AppLocalizations.of(context)!
                                              .sendFileTooltip,
                                        ),
                                      if (widget.onPickImagePressed != null)
                                        _AttachmentButton(
                                          icon: Icons.add_a_photo,
                                          onPressed: () => _sendFilePressed(
                                              widget.onPickImagePressed),
                                          tooltip: AppLocalizations.of(context)!
                                              .sendFileTooltip,
                                        ),
                                      if (widget.onSendAudioPressed != null)
                                        _AttachmentButton(
                                          icon: Icons.mic_none,
                                          onPressed: widget.onSendAudioPressed,
                                          tooltip: AppLocalizations.of(context)!
                                              .sendAudioTooltip,
                                        ),
                                    ],
                                  ),
                                ),
                                focusNode: _inputFocusNode,
                                keyboardType: TextInputType.multiline,
                                maxLines: 5,
                                minLines: 1,
                                textCapitalization:
                                    TextCapitalization.sentences,
                              ),
                            ),
                            const SizedBox(width: 16.0),
                            Visibility(
                              visible: _sendButtonVisible,
                              child: SendMessageButton(
                                  onPressed: _handleSendPressed),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
        if (widget.isDisabled && widget.disabledMessage != null)
          Container(
            color: Colors.black54,
            padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 20),
            child: Text(
              widget.disabledMessage!,
              textAlign: TextAlign.center,
              style: const TextStyle(
                color: Colors.white,
                fontStyle: FontStyle.italic,
              ),
            ),
          )
      ],
    );
  }
}

class SendMessageButton extends StatelessWidget {
  const SendMessageButton({super.key, required this.onPressed});

  final void Function() onPressed;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 24,
      height: 24,
      child: IconButton(
        icon: const Icon(Icons.send),
        splashRadius: 24,
        onPressed: onPressed,
        padding: EdgeInsets.zero,
        tooltip: AppLocalizations.of(context)!.sendTooltip,
      ),
    );
  }
}

class _AttachmentButton extends StatelessWidget {
  const _AttachmentButton({
    this.onPressed,
    this.icon = Icons.attach_file,
    this.tooltip,
  });

  final void Function()? onPressed;

  final IconData icon;

  final String? tooltip;
  @override
  Widget build(BuildContext context) {
    return Container(
      width: 24,
      height: 24,
      margin: const EdgeInsets.only(right: 16),
      child: IconButton(
        color: Theme.of(context).iconTheme.color,
        icon: Icon(icon),
        splashRadius: 24,
        onPressed: onPressed,
        padding: EdgeInsets.zero,
        tooltip: tooltip,
      ),
    );
  }
}
