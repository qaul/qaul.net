// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.
part of 'libqaul.dart';

/// initialize libqaul and communicate with libqaul's C API
///
/// The libqaul C ffi API can be found at `libqaul/src/api/c.rs`
///
/// load dynamic libqaul library and accessing libqaul's C API ffi through dart
class LibqaulFFI extends LibqaulInterface {
  static DynamicLibrary? _lib;

  final _log = Logger('LibqaulFfi');

  LibqaulFFI() {
    // check if library has already been loaded
    if (_lib != null) return;

    // check build mode (release or debug target)
    String mode;
    if (kReleaseMode) {
      mode = 'release';
    } else {
      mode = 'debug';
    }

    // load library
    if (Platform.isLinux) {
      try {
        _lib = DynamicLibrary.open('../rust/target/$mode/liblibqaul.so');
      } catch (e) {
        debugPrint("$e");
      }
    } else if (Platform.isMacOS) {
      _lib = DynamicLibrary.open('libqaul.dylib');
    } else if (Platform.isWindows) {
      var lib =
          Platform.script.resolve('libqaul.dll').toFilePath(windows: true);
      _log.config("(Windows) attempting to open libqaul.ddl from path $lib");
      _lib = DynamicLibrary.open(lib);
    } else {
      throw ('Platform ${Platform.operatingSystem} not implemented yet OR is not supported by FFI.');
    }
  }

  @override
  Future<String> getPlatformVersion() async => '';

  @override
  Future load() async => {};

  @override
  Future<void> start() async {
    _log.finer("flutter start snap libqaul");
    final start = _lib!.lookupFunction<StartFunctionRust, StartFunctionDart>(
      'start',
    );

    final path = await _getAppConfigStoragePath();

    final pathBytes = Uint8List.fromList(path.codeUnits);
    final buffer = malloc<Uint8>(pathBytes.length + 1);

    try {
      for (var i = 0; i < pathBytes.length; i++) {
        buffer[i] = pathBytes[i];
      }
      buffer[pathBytes.length] = 0;

      start(buffer);
    } catch (e) {
      debugPrint("$e");
    } finally {
      malloc.free(buffer);
    }
  }

  Future<String> _getAppConfigStoragePath() async {
    if (Platform.isLinux) {
      final env = Platform.environment;
      if (env.containsKey("SNAP_USER_COMMON")) {
        return env['SNAP_USER_COMMON']!;
      }
      if (env.containsKey("FLUTTER_ROOT") &&
          ["FLUTTER_ROOT"].contains('snap')) {
        return '${env['HOME']}/snap/flutter/common/qaul';
      }
      return '${env['HOME']}/.config/qaul';
    }
    return (await getApplicationSupportDirectory()).path;
  }

  @override
  Future<int> initialized() async {
    final initialized = _lib!
        .lookupFunction<InitializationFinishedRust, InitializationFinishedDart>(
            'initialized');
    final result = initialized();
    return result;
  }

  @override
  Future<String> hello() async {
    final hello =
        _lib!.lookupFunction<HelloFunctionRust, HelloFunctionDart>('hello');
    final ptr = hello();
    final helloMessage = ptr.toDartString();
    calloc.free(ptr);
    return helloMessage;
  }

  @override
  Future<int> checkSendCounter() async {
    final checkCounter = _lib!
        .lookupFunction<SendRpcCounterRust, SendRpcCounterDart>(
            'send_rpc_to_libqaul_count');
    final result = checkCounter();
    _log.finer("$result RPC messages sent to libqaul");
    return result;
  }

  @override
  Future<int> checkReceiveQueue() async {
    final checkQueue = _lib!
        .lookupFunction<ReceiveRpcQueuedRust, ReceiveRpcQueuedDart>(
            'receive_rpc_from_libqaul_queued_length');
    final result = checkQueue();
    if (result > 0) _log.finer("$result messages queued by libqaul RPC");
    return result;
  }

  @override
  Future<void> sendRpc(Uint8List message) async {
    final sendRpcToLibqaul = _lib!.lookupFunction<SendRpcToLibqaulFunctionRust,
        SendRpcToLibqaulFunctionDart>('send_rpc_to_libqaul');

    final buffer = malloc<Uint8>(message.length);

    try {
      for (var i = 0; i < message.length; i++) {
        buffer[i] = message[i];
      }

      final messageSize = message.length;
      _log.finer("sendRpc send $messageSize bytes");
      final result = sendRpcToLibqaul(buffer, message.length);
      _logSendRpcResult(result);
    } finally {
      malloc.free(buffer);
    }
  }

  void _logSendRpcResult(int result) {
    switch (result) {
      case 0:
        _log.finer("sendRpc success");
        break;
      case -1:
        _log.finer("sendRpc Error: pointer is null");
        break;
      case -2:
        _log.finer("sendRpc Error: message is too big");
        break;
      default:
        _log.finer("sendRpc invalid result");
        break;
    }
  }

  @override
  Future<Uint8List?> receiveRpc() async {
    final receiveRpcFromLibqaul = _lib!.lookupFunction<
        ReceiveRpcFromLibqaulFunctionRust,
        ReceiveRpcFromLibqaulFunctionDart>('receive_rpc_from_libqaul');

    const bufferSize = 259072;
    final buffer = malloc<Uint8>(bufferSize);

    Uint8List? rpcMessage;
    try {
      final result = receiveRpcFromLibqaul(buffer, bufferSize);

      if (result == 0) {
        _log.finer("receiveRpc: nothing received");
      } else if (result > 0) {
        _log.finer("receiveRpc: $result bytes received");
        rpcMessage = Uint8List.fromList(buffer.asTypedList(result));
      } else {
        _logReceiveRpcError(result);
      }
    } finally {
      malloc.free(buffer);
    }
    return rpcMessage;
  }

  void _logReceiveRpcError(int result) {
    switch (result) {
      case -1:
        _log.finer("receiveRpc ERROR -1: an error occurred");
        break;
      case -2:
        _log.finer("receiveRpc ERROR -2: buffer to small");
        break;
      case -3:
        _log.finer("receiveRpc ERROR -3: buffer pointer is null");
        break;
      default:
        _log.finer("receivedRpc unknown ERROR $result");
        break;
    }
  }
}
