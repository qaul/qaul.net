import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/stores/stores.dart';

void main() {
  group('FeedMessage (feed display and refresh)', () {
    test('forwards message fields so feed shows content and refresh uses correct index', () {
      final sendTime = DateTime(2025, 1, 15, 10, 0);
      final receiveTime = DateTime(2025, 1, 15, 10, 1);
      final message = PublicPost(
        senderId: Uint8List.fromList([1, 2, 3]),
        index: 42,
        senderIdBase58: 'sender58',
        messageId: Uint8List.fromList([4, 5, 6]),
        messageIdBase58: 'msg58',
        content: 'Hello feed',
        sendTime: sendTime,
        receiveTime: receiveTime,
      );
      final author = User(
        name: 'Author',
        id: Uint8List.fromList('author_id'.codeUnits),
      );
      const sentTimestamp = '2 min ago';

      final feedMessage = FeedMessage(message, author, sentTimestamp);

      expect(feedMessage.index, 42);
      expect(feedMessage.content, 'Hello feed');
      expect(feedMessage.author, author);
      expect(feedMessage.sentTimestamp, sentTimestamp);
    });
  });
}
