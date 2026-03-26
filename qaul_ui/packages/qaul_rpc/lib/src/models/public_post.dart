import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/services/feed/feed.pb.dart';
import 'user.dart' show PaginationState;

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

class PaginatedPosts {
  PaginatedPosts({required this.posts, this.pagination});

  final List<PublicPost> posts;
  final PaginationState? pagination;
}

class PublicPostListNotifier extends Notifier<List<PublicPost>> {
  PaginationState? _pagination;
  PaginationState? get pagination => _pagination;

  @override
  List<PublicPost> build() => [];

  void add(PublicPost message) {
    if (!contains(message)) {
      state = [message, ...state];
    }
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

  /// Replace the entire list (used for initial paginated load).
  void setAll(List<PublicPost> posts, {PaginationState? pagination}) {
    _pagination = pagination;
    state = posts;
  }

  /// Append older messages at the end (used for "load more").
  void appendOlder(List<PublicPost> posts, {PaginationState? pagination}) {
    _pagination = pagination;
    final existingIds = state.map((m) => m.messageIdBase58).toSet();
    final newPosts =
        posts.where((p) => !existingIds.contains(p.messageIdBase58)).toList();
    if (newPosts.isEmpty) return;
    state = [...state, ...newPosts];
  }

  /// Prepend newer messages at the start (used for polling sync).
  void prependNewer(List<PublicPost> posts) {
    final existingIds = state.map((m) => m.messageIdBase58).toSet();
    final newPosts =
        posts.where((p) => !existingIds.contains(p.messageIdBase58)).toList();
    if (newPosts.isEmpty) return;
    state = [...newPosts, ...state];
  }
}
