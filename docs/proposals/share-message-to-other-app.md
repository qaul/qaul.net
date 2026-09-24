# Share a chat message with another app

## Purpose

Allow a person to share the plain text of a qaul chat message with another
app installed on their device. On Android, this can be WhatsApp; on iOS, it
can be WhatsApp or another available share-sheet activity.

This is an **outgoing** share only. qaul will not receive content shared by
other apps as part of this feature.

## User experience

1. The person long-presses a text message. The existing white selection
   outline stays visible.
2. They choose **Share** in the contextual message menu.
3. qaul closes its menu and opens the operating system's share UI.
4. The person selects an installed destination and completes or dismisses
   the share there.
5. They return to the unchanged qaul conversation.

The operating system decides which destination apps appear. qaul must not
promise that WhatsApp is installed or that a third-party app accepts a
payload.

## First implementation scope

| Included | Not included |
| --- | --- |
| One text message through the native share sheet | Receiving a share from another app into qaul |
| Android and iOS | Sharing to a preselected WhatsApp contact |
| The visible message text | Attachments, images, audio, files, location, or contacts |
| Opening, dismissing, and handling errors from the sheet | Delivery/read receipts from another app |

Sharing must not change a qaul message status. A platform result can report
that the user selected an action, but cannot reliably prove delivery by a
third-party app.

## Recommended approach

Use [`share_plus`](https://pub.dev/packages/share_plus). It opens Android's
`ACTION_SEND` UI and iOS's `UIActivityViewController`; therefore qaul does
not need a WhatsApp SDK, account data, or a hardcoded app list.

Keep the plugin behind a small service. The chat widget owns interaction,
while the service can be unit-tested using a fake.

```dart
abstract interface class MessageShareService {
  Future<void> shareText({
    required String text,
    required Rect? sharePositionOrigin,
  });
}

class SystemMessageShareService implements MessageShareService {
  @override
  Future<void> shareText({
    required String text,
    required Rect? sharePositionOrigin,
  }) async {
    await SharePlus.instance.share(
      ShareParams(text: text, sharePositionOrigin: sharePositionOrigin),
    );
  }
}
```

This demonstrates the current `SharePlus.instance.share(ShareParams(...))`
API. For the current qaul toolchain, use `share_plus: ^11.1.0` rather than
the 12.x line.

## Integration outline

`qaul_ui/lib/screens/home/tabs/chat/widgets/chat.dart` already contains a
visible but disabled `share` item in `_buildForwardContextMenuElements`.
The implementation should:

1. Add an `onShare` callback next to the existing forward and copy callbacks.
2. Obtain the selected message's overlay rectangle using the existing
   `_messageRectInOverlay` pattern.
3. Dismiss qaul's contextual menu before calling the share service.
4. Pass the selected message text and that rectangle to the service.
5. Localize the Share label before enabling it.

On iPad, `sharePositionOrigin` anchors the popover to the selected message.
On phones, the same value is harmless. Blank text should no-op; a plugin
error should be reported non-blockingly without crashing the chat.

## Platform compatibility check

`share_plus` 12.0.0 raised its Android requirements to Flutter 3.38.1, iOS 13,
Java 17, Kotlin 2.2, Android Gradle Plugin 8.12.1, and Gradle 8.13.

This repository currently meets the Flutter and iOS requirements:

- Flutter 3.47.4 and Dart 3.13.3;
- iOS deployment target 13.0.

Use `share_plus: ^11.1.0` for this feature. Its published manifest requires
Flutter 3.22 and Dart 3.4, and its Android module uses Java 17, AGP 8.3.1,
Kotlin 1.7.22, and compile SDK 34. qaul's Flutter 3.47.4, Java 17, Gradle
8.11.1, AGP 8.9.1, Kotlin 2.1.0, and compile SDK 36 exceed those requirements.

The `^11.1.0` constraint allows compatible 11.x patches while preventing an
automatic upgrade to the Android-toolchain-breaking 12.x line. The first
implementation PR must still run Android and iOS builds before this is treated
as verified.

## Privacy rules

- Sharing leaves qaul's encrypted conversation boundary through an explicit
  user action.
- First version shares text only: no sender ID, room ID, timestamp, delivery
  state, encryption state, or unseen message data.
- Do not log shared text. Future telemetry may use only coarse events such
  as `share_sheet_opened` or an error category.

## Validation plan

1. Unit-test a fake service: correct text, blank-text rejection, and plugin
   errors.
2. Widget-test that Share is enabled only for shareable text messages and
   receives the selected message.
3. Manually test Android with an installed target such as WhatsApp.
4. Manually test iPhone and iPad, including the popover anchor.
5. Re-run chat and copy/forward tests to protect selection and overlay cleanup.

## Later work

Attachments require separate design and security review for local export,
temporary-file lifetime, MIME type, filenames, and policy. Receiving a share
from another app is a separate feature requiring Android intent filters and
an iOS Share Extension.

## References

- [`share_plus` package documentation](https://pub.dev/packages/share_plus)
- [Apple: sharing copies with UIActivityViewController](https://developer.apple.com/documentation/UIKit/collaborating-and-sharing-copies-of-your-data)
- [Flutter: Android share intents](https://docs.flutter.dev/flutter-for/android-devs)
