import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/providers/providers.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  late ProviderContainer container;

  setUp(() {
    container = ProviderContainer.test();
  });

  group('TabType enum regression safe-guard', () {
    test('has correct number of tabs', () {
      expect(TabType.values.length, 5);
    });

    test('tabs are in the intended order', () {
      expect(TabType.values[0], TabType.account);
      expect(TabType.values[1], TabType.public);
      expect(TabType.values[2], TabType.users);
      expect(TabType.values[3], TabType.chat);
      expect(TabType.values[4], TabType.network);
    });

    test('indices match their position', () {
      expect(TabType.account.index, 0);
      expect(TabType.public.index, 1);
      expect(TabType.users.index, 2);
      expect(TabType.chat.index, 3);
      expect(TabType.network.index, 4);
    });
  });

  group('HomeScreenTabController', () {
    group('initialization', () {
      test('initial state is TabType.public', () {
        final state = container.read(homeScreenControllerProvider);
        expect(state, TabType.public);
      });

      test('controller returns a PageController', () {
        final notifier = container.read(homeScreenControllerProvider.notifier);
        final controller = notifier.controller();

        expect(controller, isA<PageController>());
      });

      test('PageController initialPage is 1', () {
        final notifier = container.read(homeScreenControllerProvider.notifier);
        final controller = notifier.controller();

        expect(controller.initialPage, 1);
      });
    });

    group('goToTab', () {
      test('changes state to the specified tab', () {
        final notifier = container.read(homeScreenControllerProvider.notifier);

        for (final tab in TabType.values) {
          notifier.goToTab(tab);
          final state = container.read(homeScreenControllerProvider);
          expect(state, tab);
        }
      });
    });

    group('state listener', () {
      test('listeners are notified on state changes', () {
        final stateChanges = <TabType>[];

        container.listen<TabType>(
          homeScreenControllerProvider,
          (previous, next) => stateChanges.add(next),
          fireImmediately: false,
        );

        final notifier = container.read(homeScreenControllerProvider.notifier);
        notifier.goToTab(TabType.chat);
        notifier.goToTab(TabType.network);
        notifier.goToTab(TabType.account);

        expect(stateChanges, [TabType.chat, TabType.network, TabType.account]);
      });

      test('listener receives correct previous and next values', () {
        TabType? capturedPrevious;
        TabType? capturedNext;

        container.listen<TabType>(homeScreenControllerProvider, (
          previous,
          next,
        ) {
          capturedPrevious = previous;
          capturedNext = next;
        }, fireImmediately: false);

        final notifier = container.read(homeScreenControllerProvider.notifier);
        notifier.goToTab(TabType.network);

        expect(capturedPrevious, TabType.public);
        expect(capturedNext, TabType.network);
      });
    });
  });
}
