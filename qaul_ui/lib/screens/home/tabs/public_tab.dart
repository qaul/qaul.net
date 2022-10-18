part of 'tab.dart';

class _Public extends BaseTab {
  const _Public({Key? key}) : super(key: key);

  @override
  _PublicState createState() => _PublicState();
}

class _PublicState extends _BaseTabState<_Public> {
  @override
  Widget build(BuildContext context) {
    super.build(context);
    useEffect(() {
      ref.read(publicNotificationControllerProvider).initialize();
      return () {};
    }, []);

    final users = ref.watch(usersProvider);
    final messages = ref.watch(publicMessagesProvider);

    final blockedIds =
        users.where((u) => u.isBlocked ?? false).map((u) => u.idBase58);
    final filteredMessages = messages
        .where((m) => !blockedIds.contains(m.senderIdBase58 ?? ''))
        .toList();

    final refreshPublic = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      final indexes = messages.map((e) => e.index ?? 1);
      await worker.requestPublicMessages(
          lastIndex: indexes.isEmpty ? null : indexes.reduce(math.max));
    }, [UniqueKey()]);

    final l18ns = AppLocalizations.of(context);
    const bullhorn = 'assets/icons/public.svg';

    return Scaffold(
      resizeToAvoidBottomInset: true,
      floatingActionButton: FloatingActionButton.large(
        elevation: 0,
        heroTag: 'publicTabFAB',
        backgroundColor: Colors.white,
        tooltip: l18ns!.createPublicPostTooltip,
        shape: const CircleBorder(side: BorderSide(color: Colors.black)),
        onPressed: () async {
          showModalBottomSheet(
            context: context,
            isScrollControlled: true,
            barrierColor: Colors.transparent,
            builder: (context) {
              return Padding(
                padding: MediaQuery.of(context).viewInsets,
                child: _CreatePublicMessage(),
              );
            },
          );
        },
        child: SvgPicture.asset(
          bullhorn,
          width: 52,
          height: 52,
          color: Colors.grey.shade600,
        ),
      ),
      body: CronTaskDecorator(
        schedule: const Duration(milliseconds: 2500),
        callback: () async => await refreshPublic(),
        child: RefreshIndicator(
          onRefresh: () async => await refreshPublic(),
          child: EmptyStateTextDecorator(
            l18ns.emptyPublicList,
            isEmpty: filteredMessages.isEmpty,
            child: ListView.separated(
              controller: ScrollController(),
              physics: const AlwaysScrollableScrollPhysics(),
              itemCount: filteredMessages.length,
              separatorBuilder: (_, __) => const Divider(height: 12.0),
              itemBuilder: (_, i) {
                final msg = filteredMessages[i];
                var theme = Theme.of(context).textTheme;
                var sentAt = describeFuzzyTimestamp(
                  msg.sendTime,
                  locale: Locale.parse(Intl.defaultLocale ?? 'en'),
                );

                final author = users.firstWhereOrNull(
                  (u) => u.idBase58 == (msg.senderIdBase58 ?? ''),
                );
                if (author == null) return const SizedBox.shrink();

                return QaulListTile.user(
                  author,
                  content: Text(msg.content ?? '', style: theme.bodyText1),
                  trailingMetadata: Text(
                    sentAt,
                    style: theme.caption!.copyWith(fontStyle: FontStyle.italic),
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

class _CreatePublicMessage extends HookConsumerWidget {
  _CreatePublicMessage({Key? key}) : super(key: key);

  final _formKey = GlobalKey<FormFieldState>();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = useTextEditingController();
    final loading = useState(false);
    final isMounted = useIsMounted();

    final sendMessage = useCallback(() async {
      if (!(_formKey.currentState?.validate() ?? false)) return;
      loading.value = true;
      final worker = ref.read(qaulWorkerProvider);
      await worker.sendPublicMessage(controller.text.trim());
      loading.value = false;
      if (!isMounted()) return;
      Navigator.pop(context); // ignore: use_build_context_synchronously
    }, [UniqueKey()]);

    final l10n = AppLocalizations.of(context)!;
    return Container(
      height: 200,
      padding: const EdgeInsets.all(20),
      decoration: const BoxDecoration(
        border: Border(top: BorderSide(color: Colors.grey)),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Expanded(
            child: TextFormField(
              key: _formKey,
              maxLines: 15,
              autofocus: true,
              controller: controller,
              keyboardType: TextInputType.multiline,
              style: Theme.of(context).textTheme.subtitle1,
              decoration: InputDecoration(
                hintText: l10n.publicNoteHintText,
              ),
              validator: (s) {
                if (s == null || s.isEmpty) {
                  return l10n.fieldRequiredErrorMessage;
                }
                return null;
              },
            ),
          ),
          Padding(
            padding: const EdgeInsets.only(left: 12, bottom: 12),
            child: GestureDetector(
              onTap: sendMessage,
              child: const Icon(Icons.send),
            ),
          ),
        ],
      ),
    );
  }
}
