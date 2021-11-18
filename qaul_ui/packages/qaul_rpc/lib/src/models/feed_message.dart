import 'dart:typed_data';

import 'package:equatable/equatable.dart';

class FeedMessage extends Equatable {
  const FeedMessage({
    this.senderId,
    this.senderIdBase58,
    this.messageId,
    this.messageIdBase58,
    this.timeSent,
    this.timeReceived,
    this.content,
  });

  final Uint8List? senderId;
  final String? senderIdBase58;
  final Uint8List? messageId;
  final String? messageIdBase58;
  final String? timeSent;
  final String? timeReceived;
  final String? content;

  @override
  List<Object?> get props => [senderIdBase58, messageIdBase58, timeSent];
}
