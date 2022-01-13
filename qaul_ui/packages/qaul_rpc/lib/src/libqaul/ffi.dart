// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.
part of 'libqaul.dart';

/// initialize libqaul and communicate with libqaul's C API
///
/// The libqaul C ffi API can be found at `libqaul/src/api/c.rs`
///
/// load dynamic libqaul library and accessing libqaul's C API ffi through dart
class LibqaulFfi {
  final Reader read;
  static DynamicLibrary? _lib;

  /// instantiate libqaul
  /// load dynamic library and initialize it
  LibqaulFfi(this.read) {
    // check if library has already been loaded
    if (_lib != null) return;

    // check build mode (release or debug target)
    String mode;
    if(kReleaseMode) {
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
      // find the library in the rust target build folder
      _lib = DynamicLibrary.open(Platform.script.resolve('libqaul.dll').toFilePath());
    } else if (Platform.isAndroid) {
      // version 1: load liblibqaul.so directly, we use version 2 now
      //   problems:
      //     libqaul was running in the GUI thread.
      //     we got some errors from android about execution rights
      // the android libraries are copied to the android directory after build
      _lib = DynamicLibrary.open('liblibqaul.so');

      // version 2: load libqaul as AAR library & communicate via system channels
    } else if (Platform.isIOS) {
      // no path as library is statically linked library
      _lib = DynamicLibrary.process();
    } else {
      // the platform is not known
      throw('Platform ${Platform.operatingSystem} not implemented yet.');
    }
  }

  /// start and initialize libqaul
  start() {
    StartDesktopFunctionDart _start;
    // check what system we are initializing
    if (Platform.isLinux || Platform.isMacOS || Platform.isWindows) {
      debugPrint("flutter start_desktop libqaul");
      // start libqaul with finding paths to save the configuration files
      _start = _lib!.lookupFunction<StartDesktopFunctionRust, StartDesktopFunctionDart>('start_desktop');
    } else {
      debugPrint("flutter start libqaul");
      // start libqaul without path to storage location
      _start = _lib!.lookupFunction<StartFunctionRust, StartFunctionDart>('start');
    }
    _start();
  }

  /// check if libqaul finished initializing
  ///
  /// returns 1, when qaul finished initializing
  /// otherwise it returns 0
  int initialized() {
    final _initialized = _lib!.lookupFunction<InitializationFinishedRust, InitializationFinishedDart>('initialized');
    final result = _initialized();
    return result;
  }

  /// hello function
  String hello() {
    final _hello = _lib!.lookupFunction<HelloFunctionRust, HelloFunctionDart>('hello');
    final ptr = _hello();
    final helloMessage = ptr.toDartString();
    calloc.free(ptr);
    return helloMessage;
  }

  /// Debug function: how many rpc messages have been sent to libqaul
  int checkSendCounter() {
    final _checkCounter = _lib!.lookupFunction<SendRpcCounterRust, SendRpcCounterDart>('send_rpc_to_libqaul_count');
    final result = _checkCounter();
    debugPrint("$result RPC messages sent to libqaul");
    return result;
  }

  /// Debug function: How many rpc messages are queued by libqaul
  int checkReceiveQueue() {
    final _checkQueue = _lib!.lookupFunction<ReceiveRpcQueuedRust, ReceiveRpcQueuedDart>('receive_rpc_from_libqaul_queued_length');
    final result = _checkQueue();
    if (result > 0) debugPrint("$result messages queued by libqaul RPC");
    return result;
  }

  /// send binary protobuf RPC message to libqaul
  sendRpc(Uint8List message) {
    final _sendRpcToLibqaul = _lib!.lookupFunction<SendRpcToLibqaulFunctionRust, SendRpcToLibqaulFunctionDart>('send_rpc_to_libqaul');

    // create message buffer
    final buffer = malloc<Uint8>(message.length);

    // fill message into buffer
    for (var i = 0; i < message.length; i++) {
      buffer[i] = message[i];
    }
    final bufferPointer = buffer.cast<Uint8>();

    // send message
    final messageSize = message.length;
    debugPrint("sendRpc send $messageSize bytes");
    final result = _sendRpcToLibqaul(bufferPointer, message.length);

    // free buffer
    malloc.free(bufferPointer);

    // analyze result
    switch(result) {
      case 0:
        debugPrint("sendRpc success");
        break;
      case -1:
        debugPrint("sendRpc Error: pointer is null");
        break;
      case -2:
        debugPrint("sendRpc Error: message is too big");
        break;
      default:
        debugPrint("sendRpc invalid result");
        break;
    }
  }

  /// receive binary protobuf RPC message from libqaul
  Uint8List? receiveRpc() {
    final _receiveRpcFromLibqaul = _lib!.lookupFunction<ReceiveRpcFromLibqaulFunctionRust, ReceiveRpcFromLibqaulFunctionDart>('receive_rpc_from_libqaul');

    // create a buffer
    const bufferSize = 4048;
    final buffer = malloc<Uint8>(bufferSize);
    final bufferPointer = buffer.cast<Uint8>();

    // request a new message
    final result = _receiveRpcFromLibqaul(bufferPointer, bufferSize);

    // check if a message was received
    if(result == 0) {
      debugPrint("receiveRpc: nothing received");
    } else if(result > 0) {
      debugPrint("receiveRpc: $result bytes received");

      // copy buffer
      final message = buffer.asTypedList(result);

      // process message
      return message;
    } else {
      switch(result) {
        case -1:
          debugPrint("receiveRpc ERROR -1: an error occurred");
          break;
        case -2:
          debugPrint("receiveRpc ERROR -2: buffer to small");
          break;
        case -3:
          debugPrint("receiveRpc ERROR -3: buffer pointer is null");
          break;
        default:
          debugPrint("receivedRpc unknown ERROR $result");
          break;
      }
    }

    // free buffer
    malloc.free(bufferPointer);
  }
}

