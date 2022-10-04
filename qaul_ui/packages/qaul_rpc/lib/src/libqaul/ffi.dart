// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.
part of 'libqaul.dart';

/// initialize libqaul and communicate with libqaul's C API
///
/// The libqaul C ffi API can be found at `libqaul/src/api/c.rs`
///
/// load dynamic libqaul library and accessing libqaul's C API ffi through dart
class LibqaulFFI extends LibqaulInterface {
  LibqaulFFI() {
    // check build mode (release or debug target)
    String mode;
    if (kReleaseMode) {
      mode = 'release';
    } else {
      mode = 'debug';
    }

    // load library
    if (Platform.isLinux) {
      // find the library in the rust target build folder
      // TODO: target Raspberry
      _lib = DynamicLibrary.open('../rust/target/$mode/liblibqaul.so');
    } else if (Platform.isMacOS) {
      // find the library in the rust target build folder
      _lib = DynamicLibrary.open('liblibqaul.dylib');
    } else if (Platform.isWindows) {
      var lib =
          Platform.script.resolve('libqaul.dll').toFilePath(windows: true);
      _lib = DynamicLibrary.open(lib);
    } else {
      throw ('Platform ${Platform.operatingSystem} not implemented yet OR is not supported by FFI.');
    }
  }

  static late DynamicLibrary _lib;

  final _log = Logger('LibqaulFfi');

  @override
  Future load() async => {};

  @override
  Future<String> getPlatformVersion() async => '';

  @override
  Future<void> start() async {
    StartDesktopFunctionDart start;
    // check what system we are initializing
    if (Platform.isLinux || Platform.isMacOS || Platform.isWindows) {
      _log.finer("flutter start_desktop libqaul");
      // start libqaul with finding paths to save the configuration files
      start = _lib.lookupFunction<StartDesktopFunctionRust,
          StartDesktopFunctionDart>('start_desktop');
    } else {
      _log.finer("flutter start libqaul");
      // start libqaul without path to storage location
      start =
          _lib.lookupFunction<StartFunctionRust, StartFunctionDart>('start');
    }
    start();
  }

  @override
  Future<int> initialized() async {
    final initialized = _lib.lookupFunction<InitializationFinishedRust,
        InitializationFinishedDart>('initialized');
    final result = initialized();
    return result;
  }

  @override
  Future<String> hello() async => using(_hello);

  String _hello(Arena arena) {
    final hello =
        _lib.lookupFunction<HelloFunctionRust, HelloFunctionDart>('hello');
    final ptr = hello();
    final helloMessage = ptr.toDartString();
    return helloMessage;
  }

  @override
  Future<int> checkSendCounter() async {
    final checkCounter =
        _lib.lookupFunction<SendRpcCounterRust, SendRpcCounterDart>(
            'send_rpc_to_libqaul_count');
    final result = checkCounter();
    _log.finer("$result RPC messages sent to libqaul");
    return result;
  }

  @override
  Future<int> checkReceiveQueue() async {
    final checkQueue =
        _lib.lookupFunction<ReceiveRpcQueuedRust, ReceiveRpcQueuedDart>(
            'receive_rpc_from_libqaul_queued_length');
    final result = checkQueue();
    if (result > 0) _log.finer("$result messages queued by libqaul RPC");
    return result;
  }

  @override
  Future<void> sendRpc(Uint8List message) async =>
      using((a) => _sendRpc(a, message), malloc);

  void _sendRpc(Arena arena, Uint8List message) {
    final sendRpcToLibqaul = _lib.lookupFunction<SendRpcToLibqaulFunctionRust,
        SendRpcToLibqaulFunctionDart>('send_rpc_to_libqaul');

    // create message buffer
    final buffer = arena.allocate<Uint8>(message.length);

    // fill message into buffer
    for (var i = 0; i < message.length; i++) {
      buffer[i] = message[i];
    }

    // send message
    final messageSize = message.length;
    _log.finer("sendRpc send $messageSize bytes");
    final result = sendRpcToLibqaul(buffer, message.length);

    // analyze result
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
  Future<Uint8List?> receiveRpc() async => using(_receiveRpc, malloc);

  Uint8List? _receiveRpc(Arena arena) {
    final receiveRpcFromLibqaul = _lib.lookupFunction<
        ReceiveRpcFromLibqaulFunctionRust,
        ReceiveRpcFromLibqaulFunctionDart>('receive_rpc_from_libqaul');

    const size = 259072;
    final buffer = arena.allocate<Uint8>(size);
    final result = receiveRpcFromLibqaul(buffer, size);

    // check if a message was receivedRpc
    if (result == 0) {
      _log.finer("receiveRpc: nothing received");
    } else if (result > 0) {
      _log.finer("receiveRpc: $result bytes received");

      // copy buffer
      final message = buffer.asTypedList(result);

      // process message
      return message;
    } else {
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
    return null;
  }
}
