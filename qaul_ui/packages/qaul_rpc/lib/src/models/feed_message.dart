import 'dart:typed_data';

import 'package:equatable/equatable.dart';

import '../generated/services/feed/feed.pb.dart' as pb;
import '../models/feed_message.dart' as models;

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

extension FMExtension on pb.FeedMessage {
  models.FeedMessage get toModelMessage => models.FeedMessage(
    senderId: Uint8List.fromList(senderId),
    senderIdBase58: senderIdBase58,
    messageId: Uint8List.fromList(messageId),
    messageIdBase58: messageIdBase58,
    timeSent: timeSent,
    timeReceived: timeReceived,
    content: content,
  );
}
