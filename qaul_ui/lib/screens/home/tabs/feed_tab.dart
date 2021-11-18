part of 'tab.dart';

class _Feed extends BaseTab {
  const _Feed({Key? key}) : super(key: key);

  @override
  _FeedState createState() => _FeedState();
}

class _FeedState extends _BaseTabState<_Feed> {
  Future<void> refreshFeed(WidgetRef ref) async {
    if (loading.value) return;
    loading.value = true;
    await RpcFeed(ref.read).requestFeedMessages();
    await Future.delayed(const Duration(seconds: 2));

    if (!mounted) return;
    final libqaul = ref.read(libqaulProvider);

    final queued = await libqaul.checkReceiveQueue();
    if (queued > 0) await libqaul.receiveRpc();
    loading.value = false;
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    final users = ref.watch(usersProvider);
    final messages = ref.watch(feedMessagesProvider);

    final blockedIds =
        users.where((u) => u.isBlocked ?? false).map((u) => u.idBase58);
    messages.removeWhere((m) => blockedIds.contains(m.senderIdBase58 ?? ''));

    final firstLoad = useState(true);
    useMemoized(() async {
      await refreshFeed(ref);
      firstLoad.value = false;
    });

    final l18ns = AppLocalizations.of(context);
    return LoadingDecorator(
      isLoading: firstLoad.value,
      backgroundColor: Colors.transparent,
      child: Scaffold(
        floatingActionButton: FloatingActionButton(
          heroTag: 'feedTabFAB',
          tooltip: l18ns!.createFeedPostTooltip,
          onPressed: () async {
            await Navigator.push(context,
                MaterialPageRoute(builder: (_) => _CreateFeedMessage()));
            await Future.delayed(const Duration(milliseconds: 2000));
            await refreshFeed(ref);
          },
          child: const Icon(Icons.add, size: 32),
        ),
        body: RefreshIndicator(
          onRefresh: () async => await refreshFeed(ref),
          child: Stack(
            children: [
              ListView.separated(
                controller: ScrollController(),
                physics: const AlwaysScrollableScrollPhysics(),
                itemCount: messages.length,
                separatorBuilder: (_, __) => const Divider(height: 12.0),
                itemBuilder: (_, i) {
                  final msg = messages[i];
                  var theme = Theme.of(context).textTheme;
                  // TODO(brenodt): Prone to exceptions if timeSent is not parsable. Update.
                  var sentAt =
                      describeFuzzyTimestamp(DateTime.parse(msg.timeSent!));

                  final authorIdx = users.indexWhere(
                    (u) => u.idBase58 == (msg.senderIdBase58 ?? ''),
                  );
                  final author = authorIdx.isNegative ? null : users[authorIdx];

                  return ListTile(
                    leading: UserAvatar.small(user: author),
                    title: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(author?.name ?? l18ns.unknown,
                            style: theme.headline6),
                        Text(
                          sentAt,
                          style: theme.caption!
                              .copyWith(fontStyle: FontStyle.italic),
                        ),
                      ],
                    ),
                    subtitle: Text(
                      msg.content ?? '',
                      style:
                          theme.bodyText2!.copyWith(fontSize: 16, height: 1.4),
                    ),
                  );
                },
              ),
              if (messages.isEmpty) const Center(child: _EmptyFeed()),
            ],
          ),
        ),
      ),
    );
  }
}

class _EmptyFeed extends StatelessWidget {
  const _EmptyFeed({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context).textTheme;
    return ValueListenableBuilder<AdaptiveThemeMode>(
      valueListenable: AdaptiveTheme.of(context).modeChangeNotifier,
      builder: (context, value, _) {
        final isDark = value == AdaptiveThemeMode.dark;
        return IgnorePointer(
          child: Text(
            AppLocalizations.of(context)!.emptyFeedList,
            style: theme.bodyText1!
                .copyWith(color: isDark ? Colors.white30 : Colors.black38),
          ),
        );
      },
    );
  }
}

class SendMessageIntent extends Intent {
  const SendMessageIntent();
}

class ExitScreenIntent extends Intent {
  const ExitScreenIntent();
}

class _CreateFeedMessage extends HookConsumerWidget {
  _CreateFeedMessage({Key? key}) : super(key: key);

  final _formKey = GlobalKey<FormFieldState>();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = useTextEditingController();
    final loading = useState(false);
    final sendMessage = useCallback(() async {
      if (!(_formKey.currentState?.validate() ?? false)) return;
      loading.value = true;

      await RpcFeed(ref.read).sendFeedMessage(controller.text.trim());
      await Future.delayed(const Duration(seconds: 2));

      final libqaul = ref.read(libqaulProvider);

      // DEBUG: how many messages are queued by libqaul
      final queued = await libqaul.checkReceiveQueue();
      // check for rpc messages
      if (queued > 0) await libqaul.receiveRpc();

      loading.value = false;
      Navigator.pop(context);
    }, [UniqueKey()]);

    final l18ns = AppLocalizations.of(context)!;
    return LoadingDecorator(
      isLoading: loading.value,
      child: Scaffold(
        appBar: AppBar(
          elevation: 0.0,
          backgroundColor: Colors.transparent,
          leading: IconButton(
            splashRadius: 24,
            icon: const Icon(Icons.close),
            tooltip: l18ns.backButtonTooltip,
            onPressed: () => Navigator.pop(context),
          ),
        ),
        floatingActionButton: FloatingActionButton(
          heroTag: 'createFeedMessageSubscreenFAB',
          tooltip: l18ns.submitPostTooltip,
          child: const Icon(Icons.check, size: 32.0),
          onPressed: sendMessage,
        ),
        body: Shortcuts(
          shortcuts: {
            LogicalKeySet(LogicalKeyboardKey.enter, LogicalKeyboardKey.control):
                const SendMessageIntent(),
            LogicalKeySet(LogicalKeyboardKey.escape): const ExitScreenIntent(),
          },
          child: Actions(
            actions: {
              SendMessageIntent: CallbackAction<SendMessageIntent>(
                onInvoke: (SendMessageIntent intent) => sendMessage(),
              ),
              ExitScreenIntent: CallbackAction<ExitScreenIntent>(
                onInvoke: (ExitScreenIntent intent) => Navigator.pop(context),
              ),
            },
            child: Padding(
              padding: const EdgeInsets.all(40),
              child: TextFormField(
                key: _formKey,
                maxLines: 15,
                autofocus: true,
                controller: controller,
                keyboardType: TextInputType.multiline,
                style: Theme.of(context).textTheme.subtitle1,
                decoration: const InputDecoration(border: InputBorder.none),
                validator: (s) {
                  if (s == null || s.isEmpty) {
                    return l18ns.fieldRequiredErrorMessage;
                  }
                  return null;
                },
              ),
            ),
          ),
        ),
      ),
    );
  }
}
