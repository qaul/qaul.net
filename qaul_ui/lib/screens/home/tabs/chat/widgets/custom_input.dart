part of 'chat.dart';

class NewLineIntent extends Intent {
  const NewLineIntent();
}

class SendMessageIntent extends Intent {
  const SendMessageIntent();
}

class _ChatTextFooter extends StatefulWidget {
  const _ChatTextFooter({
    required this.onSendPressed,
    required this.hintText,
    this.onAttachmentPressed,
    this.onPickImagePressed,
    this.onSendAudioPressed,
    this.replyPreview,
    this.onCancelReply,
    this.disabledMessage,
    this.isDisabled = false,
  });

  final void Function(types.PartialText) onSendPressed;
  final VoidCallback? onAttachmentPressed;
  final VoidCallback? onPickImagePressed;
  final VoidCallback? onSendAudioPressed;
  final ChatFooterReplyPreviewData? replyPreview;
  final VoidCallback? onCancelReply;
  final bool isDisabled;
  final String? disabledMessage;
  final String hintText;

  @override
  State<_ChatTextFooter> createState() => _ChatTextFooterState();
}

class _ChatTextFooterState extends State<_ChatTextFooter> {
  late final TextEditingController _textController;

  @override
  void initState() {
    super.initState();
    _textController = TextEditingController();
  }

  @override
  void dispose() {
    _textController.dispose();
    super.dispose();
  }

  void _handleSend(String text) {
    if (widget.isDisabled) return;
    widget.onSendPressed(types.PartialText(text: text));
    _textController.clear();
  }

  @override
  Widget build(BuildContext context) {
    final footer = ChatFooter(
      controller: _textController,
      placeholder: widget.hintText,
      onSend: _handleSend,
      replyPreview: widget.replyPreview,
      onCancelReply: widget.onCancelReply,
      onVoicePressed: widget.isDisabled ? null : widget.onSendAudioPressed,
      onCameraPressed: widget.isDisabled ? null : widget.onPickImagePressed,
      onAttachmentPressed: widget.isDisabled ? null : widget.onAttachmentPressed,
      sendTooltip: AppLocalizations.of(context)!.sendTooltip,
      voiceTooltip: AppLocalizations.of(context)!.sendAudioTooltip,
      cameraTooltip: AppLocalizations.of(context)!.sendFileTooltip,
      attachmentsTooltip: AppLocalizations.of(context)!.sendFileTooltip,
    );

    return Stack(
      alignment: Alignment.center,
      children: [
        Opacity(
          opacity: widget.isDisabled ? 0.3 : 1,
          child: footer,
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
          ),
      ],
    );
  }
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
    this.isTextRequired = true,
  });

  final void Function(types.PartialText) onSendPressed;

  final SendButtonVisibilityMode sendButtonVisibilityMode;

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

    _textController = TextEditingController();

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

  void _handleSendPressed() {
    final trimmedText = _textController.text.trim();
    if (trimmedText != '' || !widget.isTextRequired) {
      final partialText = types.PartialText(text: trimmedText);
      widget.onSendPressed(partialText);
      _textController.clear();
    }
  }

  void _handleTextControllerChange() {
    setState(() {
      _sendButtonVisible = _textController.text.trim() != '';
    });
  }

  @override
  Widget build(BuildContext context) {
    final query = MediaQuery.of(context);

    return GestureDetector(
      onTap: () => _inputFocusNode.requestFocus(),
      child: Shortcuts(
        shortcuts: {
          LogicalKeySet(LogicalKeyboardKey.enter): const SendMessageIntent(),
          LogicalKeySet(LogicalKeyboardKey.enter, LogicalKeyboardKey.alt):
              const NewLineIntent(),
          LogicalKeySet(LogicalKeyboardKey.enter, LogicalKeyboardKey.shift):
              const NewLineIntent(),
        },
        child: Actions(
          actions: {
            SendMessageIntent: CallbackAction<SendMessageIntent>(
              onInvoke: (SendMessageIntent intent) => _handleSendPressed(),
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
                        ),
                        focusNode: _inputFocusNode,
                        keyboardType: TextInputType.multiline,
                        maxLines: 5,
                        minLines: 1,
                        textCapitalization: TextCapitalization.sentences,
                      ),
                    ),
                    const SizedBox(width: 16.0),
                    Visibility(
                      visible: _sendButtonVisible,
                      child: SendMessageButton(onPressed: _handleSendPressed),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
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
