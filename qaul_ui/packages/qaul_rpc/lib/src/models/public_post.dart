import 'dart:typed_data';

import 'package:equatable/equatable.dart';

import '../generated/services/feed/feed.pb.dart';
import 'user.dart' show PaginationState;

class PublicPost extends Equatable {
  const PublicPost({
    this.senderId,
    this.index,
    this.senderIdBase58,
    this.messageId,
    this.messageIdBase58,
    this.content,
    required this.sendTime,
    required this.receiveTime,
  });

  final Uint8List? senderId;
  final int? index;
  final String? senderIdBase58;
  final Uint8List? messageId;
  final String? messageIdBase58;
  final String? content;
  final DateTime sendTime;
  final DateTime receiveTime;

  @override
  List<Object?> get props => [senderIdBase58, messageIdBase58, index];
}

extension FMExtension on FeedMessage {
  PublicPost get toModelMessage => PublicPost(
        senderId: Uint8List.fromList(senderId),
        index: index.toInt(),
        senderIdBase58: senderIdBase58,
        messageId: Uint8List.fromList(messageId),
        messageIdBase58: messageIdBase58,
        content: content,
        sendTime: DateTime.fromMillisecondsSinceEpoch(timestampSent.toInt()),
        receiveTime:
            DateTime.fromMillisecondsSinceEpoch(timestampReceived.toInt()),
      );
}

class PaginatedPosts {
  PaginatedPosts({required this.posts, this.pagination});

  final List<PublicPost> posts;
  final PaginationState? pagination;
}
