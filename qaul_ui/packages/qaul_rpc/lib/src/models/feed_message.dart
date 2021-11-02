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

  final List<int>? senderId;
  final String? senderIdBase58;
  final List<int>? messageId;
  final String? messageIdBase58;
  final String? timeSent;
  final String? timeReceived;
  final String? content;

  @override
  List<Object?> get props => [senderIdBase58, messageIdBase58, timeSent];
}
