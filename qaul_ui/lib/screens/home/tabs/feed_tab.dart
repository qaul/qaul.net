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
        child: SvgPicture.asset(
          'assets/icons/comment.svg',
          width: 20,
          height: 20,
          color: Colors.black,
        ),
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
            return ListTile(
              leading: UserAvatar.small(),
              title: Text(
                msg.content ?? '',
                style: theme.bodyText2!.copyWith(fontSize: 16, height: 1.4),
                textAlign: TextAlign.justify,
              ),
              subtitle: Padding(
                padding: const EdgeInsets.only(top: 4.0),
                child: Text(
                  // TODO(brenodt): Prone to exceptions if timeSent is not parsable. Update.
                  timeago.format(DateTime.parse(msg.timeSent ?? '')),
                  style: theme.caption!.copyWith(fontStyle: FontStyle.italic),
                ),
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
        child: const Icon(Icons.check),
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
        ),
      ),
    );
  }
}
