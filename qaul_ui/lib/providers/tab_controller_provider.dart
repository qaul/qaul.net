part of 'providers.dart';

final tabControllerProvider = ChangeNotifierProvider((ref) => TabController());

enum TabType { account, feed, users, chat, network }

class TabController extends ChangeNotifier {
  int get currentIndex => _index;
  int _index = 0;

  @protected
  void goToIndex(int index) {
    assert(!index.isNegative && index < TabType.values.length);
    _index = index;
    notifyListeners();
  }

  void goToTab(TabType tab) => goToIndex(TabType.values.indexOf(tab));
}
