import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:local_notifications/src/local_notifications.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../test_utils/test_utils.dart';

part 'fixtures.dart';

void main() {
  late Key chatKey;

  setUp(() {
    chatKey = UniqueKey();
    SharedPreferences.setMockInitialValues({});
  });

  testResponsiveWidgets('empty state chat tab', (tester) async {
    final chatWithProviders = ProviderScope(
      overrides: [
        defaultUserProvider.overrideWithValue(
          StateController(defaultUser),
        ),
        chatNotificationControllerProvider.overrideWithValue(
          NullChatNotificationController(),
        )
      ],
      child: BaseTab.chat(
        key: chatKey,
      ),
    );

    await tester.pumpWidget(materialAppWithLocalizations(chatWithProviders));
    expect(find.byKey(chatKey), findsOneWidget);
  }, goldenCallback: (sizeName, tester) async {
    await expectLater(
      find.byKey(chatKey),
      matchesGoldenFile('goldens/chatGolden_emptyState_$sizeName.png'),
    );
  });
}
