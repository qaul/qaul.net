part of 'tab.dart';

class _Feed extends BaseTab {
  const _Feed({Key? key}) : super(key: key);

  @override
  _FeedState createState() => _FeedState();
}

class _FeedState extends _BaseTabState<_Feed> {
  @override
  Widget build(BuildContext context) {
    super.build(context);
    final users = ref.watch(usersProvider);
    final messages = ref.watch(feedMessagesProvider);

    final blockedIds =
        users.where((u) => u.isBlocked ?? false).map((u) => u.idBase58);
    final filteredMessages = messages
        .where((m) => !blockedIds.contains(m.senderIdBase58 ?? ''))
        .toList();

    final refreshFeed = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      await worker.requestFeedMessages();
    }, [UniqueKey()]);

    final l18ns = AppLocalizations.of(context);
    return Scaffold(
      floatingActionButton: FloatingActionButton(
        heroTag: 'feedTabFAB',
        tooltip: l18ns!.createFeedPostTooltip,
        onPressed: () async {
          await Navigator.push(context,
              MaterialPageRoute(builder: (_) => _CreateFeedMessage()));
          await Future.delayed(const Duration(milliseconds: 2000));
          await refreshFeed();
        },
        child: const Icon(Icons.add, size: 32),
      ),
      body: CronTaskDecorator(
        schedule: const Duration(milliseconds: 2500),
        callback: () async => await refreshFeed(),
        child: RefreshIndicator(
          onRefresh: () async => await refreshFeed(),
          child: EmptyStateTextDecorator(
            l18ns.emptyFeedList,
            isEmpty: filteredMessages.isEmpty,
            child: ListView.separated(
              controller: ScrollController(),
              physics: const AlwaysScrollableScrollPhysics(),
              itemCount: filteredMessages.length,
              separatorBuilder: (_, __) => const Divider(height: 12.0),
              itemBuilder: (_, i) {
                final msg = filteredMessages[i];
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
                    style: theme.caption,
                  ),
                );
              },
            ),
          ),
        ),
      ),
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
      final worker = ref.read(qaulWorkerProvider);
      await worker.sendFeedMessage(controller.text.trim());
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
