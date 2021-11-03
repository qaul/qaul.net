part of '../home_screen.dart';

class _FeedTab extends ConsumerWidget {
  const _FeedTab({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final messages = ref.watch(feedMessagesProvider);
    return Scaffold(
      floatingActionButton: FloatingActionButton(
        onPressed: () => Navigator.push(context,
            MaterialPageRoute(builder: (_) => const _CreateFeedMessage())),
        child: const Icon(Icons.add, size: 32),
      ),
      body: RefreshIndicator(
        onRefresh: () async => await RpcFeed(ref.read).requestFeedMessages(),
        child: ListView.separated(
          physics: const AlwaysScrollableScrollPhysics(),
          itemCount: messages.length,
          separatorBuilder: (_, __) => const Divider(height: 12.0),
          itemBuilder: (_, i) {
            final msg = messages[i];
            var theme = Theme.of(context).textTheme;
            // TODO(brenodt): Prone to exceptions if timeSent is not parsable. Update.
            var sentAt = describeFuzzyTimestamp(DateTime.parse(msg.timeSent!));

            return ListTile(
              leading: UserAvatar.small(),
              title: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  // TODO(brenodt): Once Users module is connected, this should be fetched from it.
                  Text('Name Surname', style: theme.headline6),
                  Text(
                    sentAt,
                    style: theme.caption!.copyWith(fontStyle: FontStyle.italic),
                  ),
                ],
              ),
              subtitle: Text(
                msg.content ?? '',
                style: theme.bodyText2!.copyWith(fontSize: 16, height: 1.4),
              ),
            );
          },
        ),
      ),
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
