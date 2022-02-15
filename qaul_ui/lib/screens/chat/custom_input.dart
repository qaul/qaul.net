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
    Key? key,
    this.isAttachmentUploading,
    this.onAttachmentPressed,
    required this.onSendPressed,
    this.onTextChanged,
    this.onTextFieldTap,
    required this.sendButtonVisibilityMode,
  }) : super(key: key);

  /// See [AttachmentButton.onPressed]
  final void Function()? onAttachmentPressed;

  /// Whether attachment is uploading. Will replace attachment button with a
  /// [CircularProgressIndicator]. Since we don't have libraries for
  /// managing media in dependencies we have no way of knowing if
  /// something is uploading so you need to set this manually.
  final bool? isAttachmentUploading;

  /// Will be called on [_CustomSendButton] tap. Has [types.PartialText] which can
  /// be transformed to [types.TextMessage] and added to the messages list.
  final void Function(types.PartialText) onSendPressed;

  /// Will be called whenever the text inside [TextField] changes
  final void Function(String)? onTextChanged;

  /// Will be called on [TextField] tap
  final void Function()? onTextFieldTap;

  /// Controls the visibility behavior of the [_CustomSendButton] based on the
  /// [TextField] state inside the [_CustomInput] widget.
  /// Defaults to [SendButtonVisibilityMode.editing].
  final SendButtonVisibilityMode sendButtonVisibilityMode;

  @override
  _CustomInputState createState() => _CustomInputState();
}

/// [_CustomInput] widget state
class _CustomInputState extends State<_CustomInput> {
  final _inputFocusNode = FocusNode();
  bool _sendButtonVisible = false;
  final _textController = TextEditingController();

  @override
  void initState() {
    super.initState();

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
    if (trimmedText != '') {
      final _partialText = types.PartialText(text: trimmedText);
      widget.onSendPressed(_partialText);
      _textController.clear();
    }
  }

  void _handleTextControllerChange() {
    setState(() {
      _sendButtonVisible = _textController.text.trim() != '';
    });
  }

  Widget _leftWidget() {
    if (widget.isAttachmentUploading == true) {
      return Container(
        height: 24,
        margin: const EdgeInsets.only(right: 16),
        width: 24,
        child: const CircularProgressIndicator(
          backgroundColor: Colors.transparent,
          strokeWidth: 1.5,
          valueColor: AlwaysStoppedAnimation<Color>(Colors.white),
        ),
      );
    } else {
      return AttachmentButton(onPressed: widget.onAttachmentPressed);
    }
  }

  @override
  Widget build(BuildContext context) {
    final _query = MediaQuery.of(context);

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
                final _newValue = '${_textController.text}\r\n';
                _textController.value = TextEditingValue(
                  text: _newValue,
                  selection: TextSelection.fromPosition(
                    TextPosition(offset: _newValue.length),
                  ),
                );
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
                  24 + _query.padding.left,
                  20,
                  24 + _query.padding.right,
                  20 + _query.viewInsets.bottom + _query.padding.bottom,
                ),
                child: Row(
                  children: [
                    if (widget.onAttachmentPressed != null) _leftWidget(),
                    Expanded(
                      child: ValueListenableBuilder<AdaptiveThemeMode>(
                        valueListenable:
                            AdaptiveTheme.of(context).modeChangeNotifier,
                        builder: (context, mode, _) {
                          var isDark = mode == AdaptiveThemeMode.dark;

                          return TextField(
                            controller: _textController,
                            cursorColor: isDark ? Colors.white : Colors.black,
                            decoration: InputDecoration(
                              labelText: AppLocalizations.of(context)!
                                  .chatEmptyMessageHint,
                              floatingLabelBehavior:
                                  FloatingLabelBehavior.never,
                              border: OutlineInputBorder(
                                borderRadius: BorderRadius.circular(20),
                              ),
                              focusedBorder: OutlineInputBorder(
                                borderRadius: BorderRadius.circular(20),
                                borderSide: BorderSide(
                                  color: isDark ? Colors.white : Colors.black,
                                ),
                              ),
                            ),
                            focusNode: _inputFocusNode,
                            keyboardType: TextInputType.multiline,
                            maxLines: 5,
                            minLines: 1,
                            onChanged: widget.onTextChanged,
                            onTap: widget.onTextFieldTap,
                            textCapitalization: TextCapitalization.sentences,
                          );
                        },
                      ),
                    ),
                    const SizedBox(width: 16.0),
                    Visibility(
                      visible: _sendButtonVisible,
                      child: _CustomSendButton(
                        onPressed: _handleSendPressed,
                      ),
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

class _CustomSendButton extends StatelessWidget {
  const _CustomSendButton({Key? key, required this.onPressed})
      : super(key: key);

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
