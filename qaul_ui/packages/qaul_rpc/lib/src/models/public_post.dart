import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/services/feed/feed.pb.dart';

final publicMessagesProvider =
    NotifierProvider<PublicPostListNotifier, List<PublicPost>>(
  PublicPostListNotifier.new,
);

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

class PublicPostListNotifier extends Notifier<List<PublicPost>> {
  @override
  List<PublicPost> build() => [];

  void add(PublicPost message) {
    state = [message, ...state];
  }

  bool contains(PublicPost message) {
    return !state
        .indexWhere(
          (m) =>
              m.senderIdBase58 == message.senderIdBase58 &&
              m.messageIdBase58 == message.messageIdBase58,
        )
        .isNegative;
  }
}
