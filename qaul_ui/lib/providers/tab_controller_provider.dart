part of 'providers.dart';

final tabControllerProvider = ChangeNotifierProvider((ref) => TabController());

enum TabType { account, feed, users, chat, network }

class TabController extends ChangeNotifier {
  int get currentIndex => _index;
  int _index = 0;

  void goToIndex(int index) {
    assert(!index.isNegative && index < TabType.values.length);
    _index = index;
    notifyListeners();
  }
}
