import 'package:flutter_chat_types/flutter_chat_types.dart' as types;
import 'package:qaul_rpc/qaul_rpc.dart';

class TextMessage {
  TextMessage({
    required this.idBase58,
    required this.text,
    required this.user,
  });

  final String idBase58;
  final String text;
  final User user;

  types.TextMessage toInternalMessage() {
    return types.TextMessage(
      id: idBase58,
      text: text,
      author: user.toInternalUser(),
    );
  }
}

extension UserExtension on User {
  types.User toInternalUser() {
    return types.User(
      id: idBase58,
      firstName: name
    );
  }
}
