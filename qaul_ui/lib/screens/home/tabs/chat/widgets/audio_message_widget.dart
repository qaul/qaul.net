part of 'chat.dart';

class AudioMessageWidget extends StatefulWidget {
  const AudioMessageWidget({
    Key? key,
    required this.message,
    required this.messageWidth,
    this.isDefaultUser = false,
  }) : super(key: key);

  final types.AudioMessage message;

  final int messageWidth;

  final bool isDefaultUser;

  @override
  State<AudioMessageWidget> createState() => _AudioMessageWidgetState();
}

class _AudioMessageWidgetState extends State<AudioMessageWidget> {
  double get _controlSize => (widget.messageWidth.toDouble()) / 10;

  final _audioPlayer = AudioPlayer()..setReleaseMode(ReleaseMode.stop);

  Duration? _position;

  Duration? _duration;

  String? audioPath;

  late StreamSubscription<void> _playerStateChangedSubscription;

  late StreamSubscription<Duration?> _durationChangedSubscription;

  late StreamSubscription<Duration> _positionChangedSubscription;

  Color get primaryColor => Theme.of(context).colorScheme.primary;

  Color get containerColor => Theme.of(context).colorScheme.primaryContainer;

  Color get backgroundColor => Theme.of(context).colorScheme.background;

  @override
  void initState() {
    _playerStateChangedSubscription =
        _audioPlayer.onPlayerComplete.listen((state) async {
      await stop();
    });
    _positionChangedSubscription = _audioPlayer.onPositionChanged.listen(
      (position) => setState(() {
        _position = position;
      }),
    );
    _durationChangedSubscription = _audioPlayer.onDurationChanged.listen(
      (duration) => setState(() {
        _duration = duration;
      }),
    );

    audioPath = widget.message.uri;
    _getDuration();
    _audioPlayer.setSource(_source);
    super.initState();
  }

  @override
  void dispose() {
    _playerStateChangedSubscription.cancel();
    _positionChangedSubscription.cancel();
    _durationChangedSubscription.cancel();
    _audioPlayer.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final ttheme = Theme.of(context).textTheme;

    return Container(
      padding: const EdgeInsetsDirectional.fromSTEB(16, 4, 8, 8),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.end,
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          SizedBox(height: _controlSize / 2),
          Row(
            mainAxisSize: MainAxisSize.max,
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: <Widget>[
              audioControls(),
              Expanded(child: audioSlider()),
            ],
          ),
          Padding(
            padding: const EdgeInsetsDirectional.only(end: 16),
            child: Text(
              '${_duration?.inSeconds ?? 0.0} Seconds',
              style: ttheme.labelLarge?.copyWith(
                color: backgroundColor,
                fontStyle: FontStyle.italic,
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget audioControls() {
    return ClipOval(
      child: Material(
        color: containerColor,
        child: InkWell(
          child: SizedBox(
            width: _controlSize,
            height: _controlSize,
            child: Icon(
              _audioPlayer.state == PlayerState.playing
                  ? Icons.pause
                  : Icons.play_arrow,
              color: primaryColor,
            ),
          ),
          onTap: () {
            if (_audioPlayer.state == PlayerState.playing) {
              pause();
            } else {
              play();
            }
          },
        ),
      ),
    );
  }

  Widget audioSlider() {
    bool canSetValue = false;
    final duration = _duration;
    final position = _position;

    if (duration != null && position != null) {
      canSetValue = position.inMilliseconds >= 0;
      canSetValue &= position.inMilliseconds < duration.inMilliseconds;
    }

    return Slider(
      activeColor: primaryColor,
      inactiveColor: backgroundColor,
      onChanged: (v) {
        if (duration != null) {
          final position = v * duration.inMilliseconds;
          _audioPlayer.seek(Duration(milliseconds: position.round()));
        }
      },
      value: canSetValue && duration != null && position != null
          ? position.inMilliseconds / duration.inMilliseconds
          : 0.0,
    );
  }

  Future<void> play() => _audioPlayer.play(_source);

  Future<void> pause() async {
    await _audioPlayer.pause();
    setState(() {});
  }

  Future<void> stop() async {
    await _audioPlayer.stop();
    setState(() {});
  }

  Future<void> _getDuration() async {
    await _audioPlayer.setSourceUrl(audioPath!);
    final duration = await _audioPlayer.getDuration();
    _duration = duration;
  }

  Source get _source => DeviceFileSource(audioPath!);
}
