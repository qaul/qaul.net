part of 'providers.dart';

final selectedTabProvider = Provider((ref) => SelectedTab(initialTab: 1));

enum TabType { account, feed, users, chat, network }

class SelectedTab extends StateNotifier<int> {
  SelectedTab({int? initialTab})
      : _initialTab = initialTab ?? 0,
        super(initialTab ?? 0);

  int get index => state;

  TabType get currentTab => TabType.values[state];

  get initialTab => _initialTab;
  final int _initialTab;

  @protected
  void goToIndex(int index) {
    assert(!index.isNegative && index < TabType.values.length);
    state = index;
  }

  void goToTab(TabType tab) => goToIndex(TabType.values.indexOf(tab));

  void goToNext() {
    index == TabType.values.length - 1 ? goToIndex(0) : goToIndex(index + 1);
  }

  void goToPrevious() {
    index == 0 ? goToIndex(TabType.values.length - 1) : goToIndex(index - 1);
  }
}
