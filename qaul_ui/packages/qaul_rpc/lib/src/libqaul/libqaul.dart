// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// Communicate with libqaul
///
/// This file provides a unified interface for communicating with
/// libqaul on all platforms. It chooses for each platform the correct
/// solution of initializing and communicating with libqaul.
///
/// There are several platform dependent solution on how to
/// communicate with libqaul.
///
/// 1) FFI: Load shared liblibqaul.so directly and communicate with it
///         via
/// 2) Channel: let the platform code load the library and
///             communicate with the platform via platform channels.

import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:logging/logging.dart';

part 'channel.dart';

part 'ffi.dart';

part 'fii_function_types.dart';

/// Libqaul facade provider state.
final libqaulProvider = Provider<Libqaul>((ref) => Libqaul(ref.read));

/// libqaul dart class,
/// loading dynamic libqaul library
/// and accessing libqaul's C API ffi through dart
class Libqaul {
  final Reader read;
  static bool? _initialized;
  static LibqaulChannel? _libqaulChannel;
  static LibqaulFfi? _libqaulFfi;

  /// instantiate libqaul
  /// load dynamic library and initialize it
  Libqaul(this.read) {
    // check if library has already been loaded
    if (_initialized != null) return;

    // initialize Library
    if (Platform.isAndroid || Platform.isIOS) {
      // load platform plugin
      _libqaulChannel = LibqaulChannel(read);
    } else {
      // load shared library
      _libqaulFfi = LibqaulFfi(read);
    }

    _initialized = true;
  }

  /// load libqaul
  Future load() async {
    if (Platform.isAndroid) {
      await _libqaulChannel!.load();
    } else {
      // no loading needed, as this is done
      // in the initialization
    }
  }

  /// start and initialize libqaul
  Future start() async {
    if (Platform.isAndroid || Platform.isIOS) {
      await _libqaulChannel!.start();
    } else {
      _libqaulFfi!.start();
    }
  }

  /// check if libqaul finished initializing
  ///
  /// returns 1, when qaul finished initializing
  /// otherwise it returns 0
  Future<int> initialized() async {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaulChannel!.initialized();
    } else {
      return _libqaulFfi!.initialized();
    }
  }

  /// Debug function: get Android Version
  /// returns a string of the android version from AAR library
  Future<String> getPlatformVersion() async {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaulChannel!.platformVersion();
    } else {
      return "getPlatformVersion() NOT IMPLEMENTED FOR THIS PLATFORM";
    }
  }

  /// Debug function: hello function
  Future<String> hello() async {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaulChannel!.hello();
    } else {
      return _libqaulFfi!.hello();
    }
  }

  /// Debug function: how many rpc messages have been sent to libqaul
  Future<int> checkSendCounter() async {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaulChannel!.checkSendCounter();
    } else {
      return _libqaulFfi!.checkSendCounter();
    }
  }

  /// Debug function: How many rpc messages are queued by libqaul
  Future<int> checkReceiveQueue() async {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaulChannel!.checkReceiveQueue();
    } else {
      return _libqaulFfi!.checkReceiveQueue();
    }
  }

  /// send binary protobuf RPC message to libqaul
  Future<void> sendRpc(Uint8List message) async {
    if (Platform.isAndroid || Platform.isIOS) {
      await _libqaulChannel!.sendRpc(message);
    } else {
      _libqaulFfi!.sendRpc(message);
    }
  }

  /// receive binary protobuf RPC message from libqaul
  Future<Uint8List?> receiveRpc() async {
    if (Platform.isAndroid || Platform.isIOS) {
      return _libqaulChannel!.receiveRpc();
    } else {
      return _libqaulFfi!.receiveRpc();
    }
  }
}
