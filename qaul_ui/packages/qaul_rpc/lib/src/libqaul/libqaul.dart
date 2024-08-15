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
library libqaul;

import 'dart:ffi';
import 'dart:io';

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:path_provider/path_provider.dart';

import 'libqaul_interface.dart';

part 'channel.dart';

part 'ffi.dart';

part 'fii_function_types.dart';

/// Libqaul facade provider state.
final libqaulProvider = Provider<Libqaul>((ref) => Libqaul());

/// libqaul dart class,
/// loading dynamic libqaul library
/// and accessing libqaul's C API ffi through dart
class Libqaul {
  static bool? _initialized;
  static late LibqaulInterface _libqaul;

  /// instantiate libqaul
  /// load dynamic library and initialize it
  Libqaul() {
    // check if library has already been loaded
    if (_initialized != null) return;
    _libqaul = LibqaulInterface.platform();
    _initialized = true;
  }

  /// load libqaul
  Future load() async {
    if (Platform.isAndroid) await _libqaul.load();
  }

  /// start and initialize libqaul
  Future start() async => await _libqaul.start();

  /// check if libqaul finished initializing
  ///
  /// returns 1, when qaul finished initializing
  /// otherwise it returns 0
  Future<int> initialized() async => _libqaul.initialized();

  /// Debug function: get Android Version
  /// returns a string of the android version from AAR library
  Future<String> getPlatformVersion() async => _libqaul.getPlatformVersion();

  /// Debug function: hello function
  Future<String> hello() async => _libqaul.hello();

  /// Debug function: how many rpc messages have been sent to libqaul
  Future<int> checkSendCounter() async => _libqaul.checkSendCounter();

  /// Debug function: How many rpc messages are queued by libqaul
  Future<int> checkReceiveQueue() async => _libqaul.checkReceiveQueue();

  /// send binary protobuf RPC message to libqaul
  Future<void> sendRpc(Uint8List message) async => _libqaul.sendRpc(message);

  /// receive binary protobuf RPC message from libqaul
  Future<Uint8List?> receiveRpc() async => _libqaul.receiveRpc();
}
