import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_ui/screens/home/tabs/chat/widgets/message_share_service.dart';

void main() {
  group('safeAttachmentFileName', () {
    for (final (name, expected) in [
      ('photo.jpg', 'photo.jpg'),
      ('../../Android/data/evil.jpg', 'evil.jpg'),
      ('..\\..\\evil.pdf', 'evil.pdf'),
      ('/etc/passwd', 'passwd'),
      ('bad:name?.txt', 'bad_name_.txt'),
      ('..', 'fallback.bin'),
      ('dir/', 'fallback.bin'),
      ('', 'fallback.bin'),
    ]) {
      test('"$name" becomes "$expected"', () {
        expect(
          safeAttachmentFileName(name, filePath: '/files/fallback.bin'),
          expected,
        );
      });
    }
  });

  test('attachmentSaveKind groups files by media type', () {
    expect(attachmentSaveKind('photo.JPG'), 'image');
    expect(attachmentSaveKind('clip.mp4'), 'video');
    expect(attachmentSaveKind('voice.m4a'), 'audio');
    expect(attachmentSaveKind('document.pdf'), 'file');
    expect(attachmentSaveKind('no-extension'), 'file');
  });
}
