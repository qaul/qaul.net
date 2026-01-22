part of 'chat.dart';

class ImageMessageWidget extends StatefulWidget {
  const ImageMessageWidget({
    super.key,
    required this.message,
    required this.messageWidth,
    this.isDefaultUser = false,
  });

  final types.ImageMessage message;

  final int messageWidth;

  final bool isDefaultUser;

  @override
  State<ImageMessageWidget> createState() => _ImageMessageWidgetState();
}

class _ImageMessageWidgetState extends State<ImageMessageWidget> {
  ImageProvider? _image;
  ImageStream? _stream;
  Size _size = const Size(0, 0);

  bool _isReceivingImage({Map<String, dynamic>? metadata}) {
    var isReceiving = false;
    var src = metadata ?? widget.message.metadata;
    if (src?.containsKey('messageState') ?? false) {
      final s = MessageState.fromJson(widget.message.metadata!['messageState']);
      isReceiving = s == MessageState.receiving;
    }
    return isReceiving;
  }

  @override
  void initState() {
    super.initState();

    if (_isReceivingImage()) return;
    _image = Conditional().getProvider(widget.message.uri);
    _size = Size(widget.message.width ?? 0, widget.message.height ?? 0);
  }

  @override
  void didUpdateWidget(old) {
    super.didUpdateWidget(old);
    if (_image == null && !_isReceivingImage()) {
      _size = Size(widget.message.width ?? 0, widget.message.height ?? 0);
      _image = Conditional().getProvider(widget.message.uri);
      _getImage();
    }
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (_size.isEmpty && _image != null) {
      _getImage();
    }
  }

  void _getImage() {
    final oldImageStream = _stream;
    _stream = _image?.resolve(createLocalImageConfiguration(context));
    if (_stream?.key == oldImageStream?.key) {
      return;
    }
    final listener = ImageStreamListener(_updateImage);
    oldImageStream?.removeListener(listener);
    _stream?.addListener(listener);
  }

  void _updateImage(ImageInfo info, bool _) {
    setState(() {
      _size = Size(
        info.image.width.toDouble(),
        info.image.height.toDouble(),
      );
    });
  }

  @override
  void dispose() {
    _stream?.removeListener(ImageStreamListener(_updateImage));
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    var color = widget.isDefaultUser ? Colors.lightBlue.shade700 : Colors.white;

    var style = Theme.of(context).textTheme.bodyLarge!.copyWith(
          color: widget.isDefaultUser ? Colors.white : Colors.black,
          fontSize: 17,
          fontWeight: FontWeight.w400,
        );

    Widget image;
    if (_isReceivingImage()) {
      image = Container(
        color: color,
        height: 80,
        width: 80,
        padding: const EdgeInsets.all(20),
        child: const CircularProgressIndicator(),
      );
    } else if (_size.aspectRatio == 0) {
      image = Container(color: color, height: _size.height, width: _size.width);
    } else if (_size.aspectRatio < 0.1 || _size.aspectRatio > 10) {
      image = Container(
        color: color,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              height: 64,
              margin: const EdgeInsets.all(16),
              width: 64,
              child: ClipRRect(
                borderRadius: BorderRadius.circular(15),
                child: Image(
                  fit: BoxFit.cover,
                  image: _image!,
                ),
              ),
            ),
            Flexible(
              child: Container(
                margin: const EdgeInsets.all(12),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      widget.message.name,
                      style: style,
                      textWidthBasis: TextWidthBasis.longestLine,
                    ),
                    Container(
                      margin: const EdgeInsets.only(
                        top: 4,
                      ),
                      child: Text(
                        fileSize(widget.message.size.truncate()),
                        style: style,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      );
    } else {
      image = Container(
        constraints: BoxConstraints(
          maxHeight: widget.messageWidth.toDouble(),
          minWidth: 170,
        ),
        child: AspectRatio(
          aspectRatio: _size.aspectRatio > 0 ? _size.aspectRatio : 1,
          child: Image(
            fit: BoxFit.contain,
            image: _image!,
          ),
        ),
      );
    }

    String? description = widget.message.metadata?['description'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        image,
        if (description != null && description.isNotEmpty) ...[
          const SizedBox(height: 12),
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 20),
            child: Text(description, style: style),
          ),
          const SizedBox(height: 8),
        ],
      ],
    );
  }
}
