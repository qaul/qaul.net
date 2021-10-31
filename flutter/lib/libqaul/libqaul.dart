// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// # Communicate with libqaul
///
/// This file provides a unified interface for communicating with
/// libqaul on all platforms. It chooses for each platform the correct
/// solution of initializing and communicating with libqaul.
///
/// There are several platform dependent solution on how to
/// communicate with libqaul.
///
/// 1) FFI:     Load shared liblibqaul.so directly and communicate with it
///             via the dart FFI for C. (see file ffi.dart)
/// 2) Channel: let the platform code load the library and
///             communicate with the platform via platform channels.
///             (see file channel.dart)

import 'dart:io';
import 'dart:typed_data';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'ffi.dart';
import 'channel.dart';

/// define libqaul global state
final libqaulProvider = Provider<Libqaul>((ref) => Libqaul(ref.read));

/// libqaul dart class,
/// loading dynamic libqaul library
/// and accessing libqaul's C API ffi through dart
class Libqaul {
  final Reader read;
  static bool? _initialized;
  static LibqaulChannel? _libqaul_channel;
  static LibqaulFfi? _libqaul_ffi;

  /// instantiate libqaul
  /// load dynamic library and initialize it
  Libqaul(this.read) {
    // check if library has already been loaded
    if (_initialized != null) return;

    // initialize Library
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      // load platform plugin
      _libqaul_channel = LibqaulChannel(this.read);
    } else {
      // load shared library
      _libqaul_ffi = LibqaulFfi(this.read);
    }

    _initialized = true;
  }

  /// load libqaul
  Future load() async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      await _libqaul_channel!.load();
    } else {
      // no loading needed, as this is done
      // in the initialization
    }
  }

  /// start and initialize libqaul
  Future start() async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      await _libqaul_channel!.start();
    } else {
      _libqaul_ffi!.start();
    }
  }

  /// check if libqaul finished initializing
  ///
  /// returns 1, when qaul finished initializing
  /// otherwise it returns 0
  Future<int> initialized() async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaul_channel!.initialized();
    } else {
      return _libqaul_ffi!.initialized();
    }
  }

  /// Debug function: get Android Version
  /// returns a string of the android version from AAR library
  Future<String> getPlatformVersion() async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaul_channel!.platformVersion();
    } else {
      return "getPlatformVersion() NOT IMPLEMENTED FOR THIS PLATFORM";
    }
  }

  /// Debug function: hello function
  Future<String> hello() async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaul_channel!.hello();
    } else {
      return _libqaul_ffi!.hello();
    }
  }

  /// Debug function: how many rpc messages have been sent to libqaul
  Future<int> checkSendCounter() async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaul_channel!.checkSendCounter();
    } else {
      return _libqaul_ffi!.checkSendCounter();
    }
  }

  /// Debug function: How many rpc messages are queued by libqaul
  Future<int> checkReceiveQueue() async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _libqaul_channel!.checkReceiveQueue();
    } else {
      return _libqaul_ffi!.checkReceiveQueue();
    }
  }

  /// send binary protobuf RPC message to libqaul
  Future<void> sendRpc(Uint8List message) async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      await _libqaul_channel!.sendRpc(message);
    } else {
      _libqaul_ffi!.sendRpc(message);
    }
  }

  /// receive binary protobuf RPC message from libqaul
  Future<void> receiveRpc() async {
    //if (Platform.isAndroid || Platform.isIOS || Platform.isMacOS) {
    if (Platform.isAndroid || Platform.isIOS) {
      await _libqaul_channel!.receiveRpc();
    } else {
      _libqaul_ffi!.receiveRpc();
    }
  }
}
