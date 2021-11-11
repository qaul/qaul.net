part of '../home_screen.dart';

class _FeedTab extends HookConsumerWidget {
  const _FeedTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final messages = ref.watch(feedMessagesProvider);
    useMemoized(() => refreshFeed(ref));

    return Scaffold(
      floatingActionButton: FloatingActionButton(
        onPressed: () => Navigator.push(context,
            MaterialPageRoute(builder: (_) => const _CreateFeedMessage())),
        child: const Icon(Icons.add, size: 32),
      ),
      body: RefreshIndicator(
        onRefresh: () async => await refreshFeed(ref),
        child: Stack(
          children: [
            ListView.separated(
                    physics: const AlwaysScrollableScrollPhysics(),
                    itemCount: messages.length,
                    separatorBuilder: (_, __) => const Divider(height: 12.0),
                    itemBuilder: (_, i) {
                      final msg = messages[i];
                      var theme = Theme.of(context).textTheme;
                      // TODO(brenodt): Prone to exceptions if timeSent is not parsable. Update.
                      var sentAt =
                          describeFuzzyTimestamp(DateTime.parse(msg.timeSent!));

                      return ListTile(
                        leading: UserAvatar.small(),
                        title: Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            // TODO(brenodt): Once Users module is connected, this should be fetched from it.
                            Text('Name Surname', style: theme.headline6),
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
    );
  }

  Future<void> refreshFeed(WidgetRef ref) async {
    await RpcFeed(ref.read).requestFeedMessages();
    await Future.delayed(const Duration(seconds: 2));

    final libqaul = ref.read(libqaulProvider);

    // DEBUG: how many messages are queued by libqaul
    final queued = await libqaul.checkReceiveQueue();
    // check for rpc messages
    if (queued > 0) await libqaul.receiveRpc();
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
            'No feed messages yet',
            style: theme.bodyText1!
                .copyWith(color: isDark ? Colors.white30 : Colors.black38),
          ),
        );
      },
    );
  }
}

class _CreateFeedMessage extends HookConsumerWidget {
  const _CreateFeedMessage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = useTextEditingController();

    return Scaffold(
      appBar: AppBar(
        elevation: 0.0,
        backgroundColor: Colors.transparent,
        leading: IconButton(
          icon: const Icon(Icons.close),
          onPressed: () => Navigator.pop(context),
        ),
      ),
      floatingActionButton: FloatingActionButton(
        child: const Icon(Icons.check, size: 32.0),
        onPressed: () async {
          await RpcFeed(ref.read).sendFeedMessage(controller.text);
          await Future.delayed(const Duration(seconds: 2));

          // TODO: check isMounted
          final libqaul = ref.read(libqaulProvider);

          // DEBUG: how many messages are queued by libqaul
          final queued = await libqaul.checkReceiveQueue();
          // check for rpc messages
          if (queued > 0) await libqaul.receiveRpc();

          Navigator.pop(context);
        },
      ),
      body: Padding(
        padding: const EdgeInsets.all(40),
        child: TextField(
          maxLines: 15,
          autofocus: true,
          controller: controller,
          keyboardType: TextInputType.multiline,
          style: Theme.of(context).textTheme.subtitle1,
          decoration: const InputDecoration(border: InputBorder.none),
        ),
      ),
    );
  }
}
