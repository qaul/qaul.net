import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';

import 'test_utils/test_utils.dart';

class _PublicLoadingController extends PublicNotificationController {
  _PublicLoadingController(super.ref, this.firstPage);

  final Completer<PaginatedPosts?> firstPage;

  @override
  Future<PaginatedPosts?> initializeWithFirstPage() => firstPage.future;
}

PaginatedPosts _emptyFirstPage() => PaginatedPosts(
      posts: [],
      pagination: PaginationState(
        hasMore: false,
        total: 0,
        offset: 0,
        limit: 50,
      ),
    );

void main() {
  testWidgets('shows loading instead of empty public text during first load',
      (tester) async {
    final firstPage = Completer<PaginatedPosts?>();

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          publicNotificationControllerProvider.overrideWith(
            (ref) => _PublicLoadingController(ref, firstPage),
          ),
        ],
        child: materialAppWithLocalizations(
          BaseTab.public(disablePageViewScroll: ValueNotifier(false)),
        ),
      ),
    );
    await tester.pump();

    expect(find.byType(QaulLoadingIndicator), findsOneWidget);
    expect(find.text('No public messages yet'), findsNothing);

    firstPage.complete(_emptyFirstPage());
    await tester.pump();
    await tester.pump();

    expect(find.byType(QaulLoadingIndicator), findsNothing);
    expect(find.text('No public messages yet'), findsOneWidget);
  });
}
