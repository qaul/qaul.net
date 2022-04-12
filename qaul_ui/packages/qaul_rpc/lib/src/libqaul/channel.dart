// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
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
class LibqaulChannel {
  final Reader read;
  static const MethodChannel libqaulChannel = MethodChannel('libqaul');

  final _log = Logger('LibqaulChannel');

  /// instantiate libqaul
  LibqaulChannel(this.read) {
    _log.finest("LibqaulChannel(this.read)");
  }

  /// load libqaul
  Future<void> load() async {
    _log.finest("load()");
    try {
      await libqaulChannel.invokeMethod('libqaulload');
      _log.finest("libqaulload called");
    } on PlatformException catch (e) {
      _log.warning("ERROR: Failed to load libqaul: '${e.message}'");
      rethrow;
    }
  }

  /// Test Platform Version dummy method
  Future<String> platformVersion() async {
    _log.finest("platformVersion()");

    //const MethodChannel local_channel = MethodChannel('libqaul');

    _log.finest("platformVersion() channel instantiated");

    String version;
    try {
      _log.finest("platformVersion() try");
      final result = await libqaulChannel.invokeMethod('getPlatformVersion');
      _log.finest("platformVersion() result: $result");
      version = 'Android platform version: $result';
    } on PlatformException catch (e) {
      version = "ERROR: libqaul getPlatformVersion: '${e.message}'";
      _log.warning(version);
      rethrow;
    }
    return version;
  }

  /// start and initiate libqaul
  Future<void> start() async {
    try {
      await libqaulChannel.invokeMethod('start');
    } on PlatformException catch (e) {
      _log.warning("ERROR: Failed to start libqaul: '${e.message}'");
      rethrow;
    }
  }

  /// check if libqaul finished initializing
  ///
  /// returns 1, when qaul finished initializing
  /// otherwise it returns 0
  Future<int> initialized() async {
    try {
      var init = await libqaulChannel.invokeMethod('initialized');
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

  /// hello function
  Future<String> hello() async {
    String result;
    try {
      result = await libqaulChannel.invokeMethod('hello');
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul hello: '${e.message}'");
      rethrow;
    }
    return result;
  }

  /// Debug function: how many rpc messages have been sent to libqaul
  Future<int> checkSendCounter() async {
    int result;
    try {
      result = await libqaulChannel.invokeMethod('sendcounter');
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul sendcounter: '${e.message}'");
      rethrow;
    }
    return result;
  }

  /// Debug function: How many rpc messages are queued by libqaul
  Future<int> checkReceiveQueue() async {
    int result;
    try {
      result = await libqaulChannel.invokeMethod('receivequeue');
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul channel receivequeue: '${e.message}'");
      rethrow;
    }
    return result;
  }

  /// send binary protobuf RPC message to libqaul
  Future<void> sendRpc(Uint8List message) async {
    try {
      await libqaulChannel.invokeMethod('sendRpcMessage', {'message': message});
    } on PlatformException catch (e) {
      _log.warning("ERROR: libqaul channel sendRpcMessage: '${e.message}'");
      rethrow;
    }
  }

  /// receive binary protobuf RPC message from libqaul
  /// and pass it to RPC module
  Future<Uint8List?> receiveRpc() async {
    try {
      final Uint8List? result = await libqaulChannel.invokeMethod('receiveRpcMessage');

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
