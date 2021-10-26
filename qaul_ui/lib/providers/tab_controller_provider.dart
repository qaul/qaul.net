part of 'providers.dart';

final selectedTabProvider = Provider((ref) => SelectedTab());

enum TabType { account, feed, users, chat, network }

class SelectedTab extends StateNotifier<int> {
  SelectedTab() : super(0);

  int get index => state;

  @protected
  void goToIndex(int index) {
    assert(!index.isNegative && index < TabType.values.length);
    state = index;
  }

  void goToTab(TabType tab) => goToIndex(TabType.values.indexOf(tab));
}
