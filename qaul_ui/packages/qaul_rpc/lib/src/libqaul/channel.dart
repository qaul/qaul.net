// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.
part of 'libqaul.dart';

/// Communicate via Platform Channel with libqaul
///
/// Libqaul shared library is not loaded directly, but loaded
/// by a platform plugin.
/// We communicate via a platform channel with the platform
/// plugin that invokes libqaul.
///
/// This procedure is used on Android.
class LibqaulChannel extends LibqaulInterface {
  LibqaulChannel() {
    _log.finest("LibqaulChannel(this.read)");
  }

  static const MethodChannel _channel = MethodChannel('libqaul');

  final _log = Logger('LibqaulChannel');

  @override
  Future<void> load() async {
    _log.finest("load()");
    try {
      await _channel.invokeMethod('libqaulload');
      _log.finest("libqaulload called");
    } on PlatformException catch (e) {
      _log.warning("ERROR: Failed to load libqaul: '${e.message}'");
      rethrow;
    }
  }

  @override
  Future<String> getPlatformVersion() async {
    _log.finest("platformVersion()");

    String version;
    try {
      _log.finest("platformVersion() try");
      final result = await _channel.invokeMethod('getPlatformVersion');
      _log.finest("platformVersion() result: $result");
      version = 'Android platform version: $result';
    } on PlatformException catch (e) {
      version = "ERROR: libqaul getPlatformVersion: '${e.message}'";
      _log.warning(version);
      rethrow;
    }
    return version;
  }

  @override
  Future<void> start() async {
    try {
      await _channel.invokeMethod('start');
    } on PlatformException catch (e) {
      _log.warning("ERROR: Failed to start libqaul: '${e.message}'");
      rethrow;
    }
  }

  @override
  Future<int> initialized() async {
    try {
      var init = await _channel.invokeMethod('initialized');
      if ((init is int && init == 1) || (init is bool && init)) {
        return 1;
      } else {
        return 0;
      }
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul initialized: '${e.message}'");
      rethrow;
    }
  }

  @override
  Future<String> hello() async {
    String result;
    try {
      result = await _channel.invokeMethod('hello');
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul hello: '${e.message}'");
      rethrow;
    }
    return result;
  }

  @override
  Future<int> checkSendCounter() async {
    int result;
    try {
      result = await _channel.invokeMethod('sendcounter');
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul sendcounter: '${e.message}'");
      rethrow;
    }
    return result;
  }

  @override
  Future<int> checkReceiveQueue() async {
    int result;
    try {
      result = await _channel.invokeMethod('receivequeue');
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul channel receivequeue: '${e.message}'");
      rethrow;
    }
    return result;
  }

  @override
  Future<void> sendRpc(Uint8List message) async {
    try {
      await _channel.invokeMethod('sendRpcMessage', {'message': message});
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul channel sendRpcMessage: '${e.message}'");
      rethrow;
    }
  }

  @override
  Future<Uint8List?> receiveRpc() async {
    try {
      final Uint8List? result =
          await _channel.invokeMethod('receiveRpcMessage');

      if (result == null) {
        _log.finest("channel receiveRpcMessage: null received");
      } else {
        _log.finest("channel receiveRpcMessage: received");

        if (result.isEmpty) {
          _log.finest("channel receiveRpcMessage: result is empty");
          return null;
        }

        // check result size
        final size = result.lengthInBytes;
        _log.finest("channel receiveRpcMessage: $size bytes received");

        if (size == 0) {
          _log.finest("channel receiveRpcMessage: size == 0");
          return null;
        }

        // decode protobuf message
        return result;
        // TODO: Free message buffer?
      }
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul receiveRpcMessage: '${e.message}'");
      rethrow;
    }
    return null;
  }
}
