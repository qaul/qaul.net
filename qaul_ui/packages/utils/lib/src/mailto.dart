/// [Mailto] helps you create email ("mailto") links.
///
/// The class takes care of all the necessary encoding.
///
/// **Flutter info**
///
/// You can use the `url_launcher` package for launching the
/// URL that the [Mailto] class's [toString] method returns.
/// It will open the default email client on the smart phone,
/// with pre-filled [to], [cc], [bcc] recipient list, [subject], and [body]
/// (content of the email).
///
/// The user can still decide not to send an email or edit any of the
/// fields.
class Mailto {
  /// Create a [Mailto] instance.
  ///
  /// Convert the instance to `String` in order to get the `mailto` URL
  /// corresponding to the instance.
  ///
  /// Fields aren't validated. Check [Mailto.validateParameters] for more info.
  Mailto({
    this.to,
    this.cc,
    this.bcc,
    this.subject,
    this.body,
  });

  /// Validate the incoming parameters whether they would be valid for
  /// a mailto link.
  ///
  /// In case the parameters don't pass validation, [ArgumentError] is thrown.
  ///
  /// The [Mailto] class does not validate its fields neither at instatiation
  /// nor when the toString method is called, so make sure that you either
  /// know that the values are valid and the mailto links work on devices,
  /// or call the `validateParameters` function in an `assert` call to catch
  /// issues during development.
  static void validateParameters({
    List<String>? to,
    List<String>? cc,
    List<String>? bcc,
    String? subject,
    String? body,
  }) {
    bool isEmptyString(String e) => e.isEmpty;
    bool containsLineBreak(String e) => e.contains('\n');
    if (to?.any(isEmptyString) == true) {
      throw ArgumentError.value(
        to,
        'to',
        'elements in "to" list must not be empty',
      );
    }
    if (to?.any(containsLineBreak) == true) {
      throw ArgumentError.value(
        to,
        'to',
        'elements in "to" list must not contain line breaks',
      );
    }
    if (cc?.any(isEmptyString) == true) {
      throw ArgumentError.value(
        cc,
        'cc',
        'elements in "cc" list must not be empty. ',
      );
    }
    if (cc?.any(containsLineBreak) == true) {
      throw ArgumentError.value(
        cc,
        'cc',
        'elements in "cc" list must not contain line breaks',
      );
    }
    if (bcc?.any(isEmptyString) == true) {
      throw ArgumentError.value(
        bcc,
        'bcc',
        'elements in "bcc" list must not be empty. ',
      );
    }
    if (bcc?.any(containsLineBreak) == true) {
      throw ArgumentError.value(
        bcc,
        'bcc',
        'elements in "bcc" list must not contain line breaks',
      );
    }
    if (subject?.contains('\n') == true) {
      throw ArgumentError.value(
        subject,
        'subject',
        '"subject" must not contain line breaks',
      );
    }
  }

  /// Main recipient(s) of your email
  ///
  /// Destination email addresses.
  final List<String>? to;

  /// Recipient(s) of a copy of the email.
  ///
  /// CC stands for carbon copy. When you CC people on an email, the CC list is
  /// visible to all other recipients.
  final List<String>? cc;

  /// Recipient(s) of a secret copy of the email.
  ///
  /// BCC stands for blind carbon copy. When you BCC people on an email, the
  /// BCC list is not visible to other recipients.
  final List<String>? bcc;

  /// Subject of email.
  final String? subject;

  /// Body of email.
  ///
  /// The content of the email.
  ///
  /// Please be aware that not all email clients are able to handle
  /// line-breaks in the body.
  final String? body;

  /// Percent encoded value of the comma (',') character.
  ///
  /// ```dart
  /// Uri.encodeComponent(',') == _comma; // true
  /// ```
  static const String _comma = '%2C';

  String _encodeTo(String s) {
    final atSign = s.lastIndexOf('@');
    return Uri.encodeComponent(s.substring(0, atSign)) + s.substring(atSign);
  }

  @override
  String toString() {
    // Use a string buffer as input is of unknown length.
    final stringBuffer = StringBuffer('mailto:');
    if (to != null) stringBuffer.writeAll(to!.map(_encodeTo), _comma);
    // We need this flag to know whether we should use & or ? when creating
    // the string.
    var parameterAdded = false;
    final parameterMap = {
      'subject': subject,
      'body': body,
      'cc': cc?.join(','),
      'bcc': bcc?.join(','),
    };
    for (final parameter in parameterMap.entries) {
      // Do not add key-value pair where the value is missing or empty
      if (parameter.value == null || parameter.value!.isEmpty) continue;
      // We don't need to encode the keys because all keys are under the
      // package's control currently and all of those keys are simple keys
      // without any special characters.
      // The values need to be encoded.
      // The RFC also mentions that the body should use '%0D%0A' for
      // line-breaks, however,we didn't find any difference between
      // '%0A' and '%0D%0A', so we keep it at '%0A'.
      stringBuffer
        ..write(parameterAdded ? '&' : '?')
        ..write(parameter.key)
        ..write('=')
        ..write(Uri.encodeComponent(parameter.value!));
      parameterAdded = true;
    }
    return stringBuffer.toString();
  }
}
