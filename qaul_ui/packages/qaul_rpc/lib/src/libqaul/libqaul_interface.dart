import 'dart:io';
import 'dart:typed_data';

import 'package:logging/logging.dart';

import 'libqaul.dart';

abstract class LibqaulInterface {
  LibqaulInterface();

  factory LibqaulInterface.platform() {
    if (Platform.isAndroid || Platform.isIOS) {
      return LibqaulChannel();
    } else if (Platform.isLinux || Platform.isMacOS || Platform.isWindows) {
      return LibqaulFFI();
    }
    _log.severe('Platform not supported by libqaul');
    return NullLibqaul();
  }

  static final _log = Logger('LibqaulInterface');

  /// load libqaul. Only required on Android
  Future load();

  /// start and initialize libqaul
  Future<void> start();

  /// check if libqaul finished initializing
  ///
  /// returns 1, when qaul finished initializing
  /// otherwise it returns 0
  Future<int> initialized();

  /// [Android] Debug function: get Android Version
  /// returns a string of the android version from AAR library
  ///
  /// [Other platforms]: yields an empty string
  Future<String> getPlatformVersion();

  /// Debug function: hello function
  Future<String> hello();

  /// Check how many rpc messages have been sent to libqaul
  Future<int> checkSendCounter();

  /// Check how many rpc messages are queued by libqaul
  Future<int> checkReceiveQueue();

  /// send binary protobuf RPC message to libqaul
  Future<void> sendRpc(Uint8List message);

  /// receive binary protobuf RPC message from libqaul
  Future<Uint8List?> receiveRpc();
}

class NullLibqaul implements LibqaulInterface {

  @override
  Future<int> checkReceiveQueue() async => 0;

  @override
  Future<int> checkSendCounter() async => 0;

  @override
  Future<String> hello() async => '';

  @override
  Future<int> initialized() async => 0;

  @override
  Future<Uint8List?> receiveRpc() async => null;

  @override
  Future<void> sendRpc(Uint8List message) async {}

  @override
  Future<void> start() async {}

  @override
  Future<String> getPlatformVersion() async => '';

  @override
  Future<void> load() async {}
}
