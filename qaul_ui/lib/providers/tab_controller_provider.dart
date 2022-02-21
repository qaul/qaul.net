part of 'providers.dart';

final selectedTabProvider = Provider((ref) => SelectedTab(initialTab: 1));

enum TabType { account, feed, users, chat, network }

@immutable
class SelectedTabStatus {
  const SelectedTabStatus({required this.tab, this.shouldScroll = true});

  final int tab;
  final bool shouldScroll;

  SelectedTabStatus copyWith({int? tab, bool? shouldScroll}) {
    return SelectedTabStatus(
      tab: tab ?? this.tab,
      shouldScroll: shouldScroll ?? this.shouldScroll,
    );
  }
}

class SelectedTab extends StateNotifier<SelectedTabStatus> {
  SelectedTab({int? initialTab})
      : _initialTab = initialTab ?? 0,
        super(SelectedTabStatus(tab: initialTab ?? 0));

  int get index => state.tab;

  TabType get currentTab => TabType.values[state.tab];

  get initialTab => _initialTab;
  final int _initialTab;

  @protected
  void goToIndex(int index, {bool scroll = true}) {
    assert(!index.isNegative && index < TabType.values.length);
    state = state.copyWith(tab: index, shouldScroll: scroll);
  }

  void goToTab(TabType tab) => goToIndex(TabType.values.indexOf(tab));

  void goToNext() {
    index == TabType.values.length - 1 ? goToIndex(0) : goToIndex(index + 1);
  }

  void goToPrevious() {
    index == 0 ? goToIndex(TabType.values.length - 1) : goToIndex(index - 1);
  }

  void updateCurrentIndexWithoutScrolling(int i) => goToIndex(i, scroll: false);
}
