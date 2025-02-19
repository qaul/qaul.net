part of 'chat.dart';

class _RecordAudioDialog extends StatefulHookConsumerWidget {
  const _RecordAudioDialog({
    required this.room,
    required this.onSendPressed,
    this.partialMessage,
  });
  final ChatRoom room;
  final String? partialMessage;
  final Function(File f, types.PartialText desc) onSendPressed;

  @override
  ConsumerState<_RecordAudioDialog> createState() => _RecordAudioDialogState();
}

class _RecordAudioDialogState extends ConsumerState<_RecordAudioDialog> {
  static final kDefaultCodecSettings = Platform.isWindows
      ? const RecordConfig(numChannels: 1, bitRate: 96000, sampleRate: 44100)
      : const RecordConfig(
          numChannels: 1,
          // 44100 / 2
          sampleRate: 22050,
          // 128000 / 2
          bitRate: 64000,
        );

  final _log = Logger('RecordAudioDialog');

  final audioPlayer = AudioPlayer();
  final audioRecorder = AudioRecorder();

  bool isRecording = false;
  File? file;
  String? audioPath;
  late ComplexTimer _timer;
  int _duration = 0;

  void incrementDuration(ComplexTimer _) {
    setState(() => _duration += 1);
    _timer.restart();
  }

  void onStopPressed() async {
    _timer.pause();
    await stopRecording();
    if (audioPath != null) {
      setState(() => file = File(audioPath!));
    }
  }

  void onCancelPressed() async {
    if (isRecording) {
      await stopRecording();
    }
    file?.deleteSync();
    if (mounted) Navigator.pop(context);
  }

  void onSendPressed() {
    final worker = ref.read(qaulWorkerProvider);
    worker.sendFile(
        pathName: file!.path,
        conversationId: widget.room.conversationId,
        description: 'audio message');

    Navigator.pop(context);
  }

  @override
  void initState() {
    super.initState();
    _timer = ComplexTimer(const Duration(seconds: 1));
    _timer.onTimeout = incrementDuration;
    _timer.pause();

    WidgetsBinding.instance.addPostFrameCallback((_) {
      startRecording();
      _timer.restart();
    });
  }

  @override
  void dispose() {
    _timer.cancel();
    audioPlayer.dispose();
    audioRecorder.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Padding(
            padding: const EdgeInsets.all(12),
            child: IconButtonFactory.close(onPressed: onCancelPressed),
          ),
          if (isRecording)
            Column(
              mainAxisSize: MainAxisSize.min,
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.mic_rounded,
                      size: 28,
                      color: Colors.redAccent.shade100,
                    ),
                    Text(
                      _messageDuration,
                    ),
                  ],
                ),
                IconButton(
                  iconSize: 48,
                  onPressed: onStopPressed,
                  color: Colors.redAccent.shade100,
                  icon: const Icon(Icons.stop_circle_outlined),
                ),
              ],
            ),
          if (file != null) ...[
            Row(
              children: [
                const SizedBox(width: 20),
                const Icon(Icons.insert_drive_file_outlined, size: 40),
                const SizedBox(width: 8),
                Flexible(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(filesize(file!.lengthSync())),
                      Text(
                        _messageDuration,
                        maxLines: 2,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ],
                  ),
                ),
              ],
            ),
            Padding(
              padding: const EdgeInsets.only(top: 20),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  OutlinedButton(
                    onPressed: onCancelPressed,
                    child: const Text('Cancel'),
                  ),
                  FilledButton(
                    onPressed: onSendPressed,
                    child: const Text('Send'),
                  ),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }

  String get _messageDuration => Duration(seconds: _duration)
      .toString()
      .substring(2)
      .replaceAll('.000000', '');

  Future<void> startRecording() async {
    try {
      if (await audioRecorder.hasPermission()) {
        setState(() {
          isRecording = true;
        });
        final path = await getNewAudioFilePath();
        await audioRecorder.start(kDefaultCodecSettings, path: path);
      }
    } catch (e) {
      _log.severe("could not start recording: $e", e, StackTrace.current);
      return;
    }
  }

  Future<void> stopRecording() async {
    try {
      final path = await audioRecorder.stop();
      setState(() {
        isRecording = false;
        audioPath = path!;
      });
    } catch (e) {
      _log.severe("could not stop recording: $e", e, StackTrace.current);
      return;
    }
  }

  Future<String> getNewAudioFilePath() async {
    final dir = (Platform.isAndroid)
        ? (await getExternalStorageDirectory())
        : (await getApplicationSupportDirectory());

    if (dir == null) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(SnackBar(
          content: Text(AppLocalizations.of(context)!.genericErrorMessage),
        ));
      }
      return "";
    }

    return join(dir.path, 'audio_${DateTime.now().millisecondsSinceEpoch}.m4a');
  }
}
