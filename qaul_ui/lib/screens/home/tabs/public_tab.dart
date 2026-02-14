part of 'tab.dart';

class _Public extends BaseTab {
  const _Public({super.key, required this.disablePageViewScroll});

  final ValueNotifier<bool> disablePageViewScroll;

  @override
  _PublicState createState() => _PublicState();
}

class _PublicState extends _BaseTabState<_Public> {
  @override
  Widget build(BuildContext context) {
    super.build(context);

    return Navigator(
      initialRoute: 'public/main',
      onGenerateRoute: (RouteSettings settings) {
        WidgetBuilder builder;
        switch (settings.name) {
          case 'public/main':
            builder = (BuildContext context) =>
                _PublicTabView(widget.disablePageViewScroll);
            break;
          default:
            throw Exception('Invalid route: ${settings.name}');
        }
        return MaterialPageRoute<void>(builder: builder, settings: settings);
      },
    );
  }
}

class _PublicTabView extends HookConsumerWidget {
  const _PublicTabView(this.disablePageViewScroll);

  final ValueNotifier<bool> disablePageViewScroll;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    useEffect(() {
      ref.read(publicNotificationControllerProvider).initialize();
      return () {};
    }, []);

    final messages = ref.watch(feedMessageStoreProvider);

    final l10n = AppLocalizations.of(context)!;
    const bullhorn = 'assets/icons/public.svg';

    final onCreatePublicMessagePressed = useCallback(() async {
      disablePageViewScroll.value = true;
      await showModalBottomSheet(
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
      disablePageViewScroll.value = false;
    }, []);

    final refreshPublic = useCallback(() async {
      await ref.read(feedMessageStoreProvider.notifier).refreshPublic();
    }, []);

    return Scaffold(
      resizeToAvoidBottomInset: true,
      floatingActionButton: QaulFAB(
        onPressed: onCreatePublicMessagePressed,
        svgAsset: bullhorn,
        heroTag: 'publicTabFAB',
        tooltip: l10n.createPublicPostTooltip,
      ),
      body: CronTaskDecorator(
        schedule: const Duration(milliseconds: 2500),
        callback: () async => await refreshPublic(),
        child: RefreshIndicator(
          onRefresh: () async => await refreshPublic(),
          child: EmptyStateTextDecorator(
            l10n.emptyPublicList,
            isEmpty: messages.isEmpty,
            child: ListView.separated(
              controller: ScrollController(),
              physics: const AlwaysScrollableScrollPhysics(),
              itemCount: messages.length,
              separatorBuilder: (_, _) => const Divider(height: 12.0),
              itemBuilder: (_, i) {
                final msg = messages[i];
                var theme = Theme.of(context).textTheme;
                return QaulListTile.user(
                  msg.author,
                  useUserColorOnName: true,
                  isContentSelectable: true,
                  content: Text(msg.content ?? '', style: theme.bodyLarge),
                  trailingMetadata: Text(
                    msg.sentTimestamp,
                    style: theme.bodySmall!.copyWith(
                      fontStyle: FontStyle.italic,
                    ),
                  ),
                  nameTapRoutesToDetailsScreen: true,
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
  _CreatePublicMessage();

  final _formKey = GlobalKey<FormFieldState>();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = useTextEditingController();
    final loading = useState(false);

    final sendMessage = useCallback(() async {
      if (!(_formKey.currentState?.validate() ?? false)) return;
      loading.value = true;
      await ref
          .read(feedMessageStoreProvider.notifier)
          .sendMessage(controller.text.trim());
      loading.value = false;
      if (!context.mounted) return;
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
              style: Theme.of(context).textTheme.titleMedium,
              decoration: InputDecoration(hintText: l10n.publicNoteHintText),
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
