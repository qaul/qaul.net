import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/services/feed/feed.pb.dart';
import 'pagination.dart';

final publicMessagesProvider =
    NotifierProvider<PublicPostListNotifier, PaginatedData<PublicPost>>(
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

class PublicPostListNotifier extends Notifier<PaginatedData<PublicPost>> {
  @override
  PaginatedData<PublicPost> build() =>
      const PaginatedData(data: [], pagination: null);

  void add(PublicPost message) {
    final data = [message, ...state.data];
    state = PaginatedData(data: data, pagination: state.pagination);
  }

  bool contains(PublicPost message) {
    return state.data.any(
      (m) =>
          m.senderIdBase58 == message.senderIdBase58 &&
          m.messageIdBase58 == message.messageIdBase58,
    );
  }

  void replaceAll(List<PublicPost> items, {PaginationState? pagination}) {
    state = PaginatedData(
      data: items,
      pagination: pagination ?? state.pagination,
    );
  }

  void appendMany(List<PublicPost> items) {
    final existingIds = state.data
        .map((m) => '${m.senderIdBase58 ?? ''}_${m.messageIdBase58 ?? ''}')
        .toSet();
    final newItems = items.where(
      (m) => !existingIds.contains('${m.senderIdBase58 ?? ''}_${m.messageIdBase58 ?? ''}'),
    ).toList();
    if (newItems.isEmpty) return;
    state = PaginatedData(
      data: [...state.data, ...newItems],
      pagination: state.pagination,
    );
  }

  void setPagination(PaginationState? pagination) {
    state = PaginatedData(data: state.data, pagination: pagination);
  }
}
