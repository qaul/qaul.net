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

    _lookupFunctions();
  }

  static late DynamicLibrary _lib;

  final _log = Logger('LibqaulFFI');

  static late final StartDesktopFunctionDart _startFn;
  static late final InitializationFinishedDart _initializeFn;
  static late final HelloFunctionDart _helloFn;
  static late final SendRpcCounterDart _sendRpcCounterFn;
  static late final ReceiveRpcQueuedDart _receiveRpcCounterFn;
  static late final SendRpcToLibqaulFunctionDart _sendRpcFn;
  static late final ReceiveRpcFromLibqaulFunctionDart _receiveRpcFn;

  static void _lookupFunctions() {
    _startFn =
        _lib.lookupFunction<StartDesktopFunctionRust, StartDesktopFunctionDart>(
            'start_desktop');
    _initializeFn = _lib.lookupFunction<InitializationFinishedRust,
        InitializationFinishedDart>('initialized');
    _helloFn =
        _lib.lookupFunction<HelloFunctionRust, HelloFunctionDart>('hello');
    _sendRpcCounterFn =
        _lib.lookupFunction<SendRpcCounterRust, SendRpcCounterDart>(
            'send_rpc_to_libqaul_count');
    _receiveRpcCounterFn =
        _lib.lookupFunction<ReceiveRpcQueuedRust, ReceiveRpcQueuedDart>(
            'receive_rpc_from_libqaul_queued_length');
    _sendRpcFn = _lib.lookupFunction<SendRpcToLibqaulFunctionRust,
        SendRpcToLibqaulFunctionDart>('send_rpc_to_libqaul');
    _receiveRpcFn = _lib.lookupFunction<ReceiveRpcFromLibqaulFunctionRust,
        ReceiveRpcFromLibqaulFunctionDart>('receive_rpc_from_libqaul');
  }

  @override
  Future load() async => {};

  @override
  Future<String> getPlatformVersion() async => '';

  @override
  Future<void> start() async => _startFn();

  @override
  Future<int> initialized() async => _initializeFn();

  @override
  Future<String> hello() async => using(_hello);

  String _hello(Arena arena) {
    final ptr = _helloFn();
    final helloMessage = ptr.toDartString();
    return helloMessage;
  }

  @override
  Future<int> checkSendCounter() async {
    final result = _sendRpcCounterFn();
    _log.finer("$result RPC messages sent to libqaul");
    return result;
  }

  @override
  Future<int> checkReceiveQueue() async {
    final result = _receiveRpcCounterFn();
    if (result > 0) _log.finer("$result messages queued by libqaul RPC");
    return result;
  }

  @override
  Future<void> sendRpc(Uint8List message) async =>
      using((a) => _sendRpc(a, message), malloc);

  void _sendRpc(Arena arena, Uint8List message) {
    final buffer = arena.allocate<Uint8>(message.length);
    for (var i = 0; i < message.length; i++) {
      buffer[i] = message[i];
    }

    final messageSize = message.length;
    _log.finer("sendRpc send $messageSize bytes");
    final result = _sendRpcFn(buffer, message.length);
    if (result != 0) _log.warning('sendRpc Error; received result: $result');
  }

  @override
  Future<Uint8List?> receiveRpc() async => using(_receiveRpc, malloc);

  Uint8List? _receiveRpc(Arena arena) {
    const size = 259072;
    final buffer = arena.allocate<Uint8>(size);
    final result = _receiveRpcFn(buffer, size);

    if (result == 0) {
      _log.finer("receiveRpc: nothing received");
    } else if (result > 0) {
      _log.finer("receiveRpc: $result bytes received");
      return buffer.asTypedList(result);
    } else {
      _log.warning('receiveRpc Error; received result: $result');
    }
    return null;
  }
}
