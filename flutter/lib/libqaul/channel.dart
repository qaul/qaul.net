// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// Communicate via Platform Channel with libqaul
///
/// Libqaul shared library is not loaded directly, but loaded
/// by a platform plugin.
/// We communicate via a platform channel with the platform
/// plugin that invokes libqaul.
///
/// This procedure is used on Android.

import 'dart:io';
import 'dart:typed_data';
import 'dart:async';
import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '/rpc/protobuf.dart';

/// define libqaul channel global state
final libqaulChannelProvider = Provider<LibqaulChannel>((ref) => LibqaulChannel(ref.read));

class LibqaulChannel {
  final Reader read;
  static const MethodChannel libqaul_channel = MethodChannel('libqaul');

  /// instantiate libqaul
  LibqaulChannel(this.read) {
    debugPrint("LibqaulChannel(this.read)");
  }

  /// load libqaul
  Future<void> load() async {
    debugPrint("load()");
    try {
      await libqaul_channel.invokeMethod('libqaulload');
      debugPrint("libqaulload called");
    } on PlatformException catch (e) {
      debugPrint("ERROR: Failed to load libqaul: '${e.message}'");
    }
  }

  /// Test Platform Version dummy method
  Future<String> platformVersion() async {
    debugPrint("platformVersion()");

    //const MethodChannel local_channel = MethodChannel('libqaul');

    debugPrint("platformVersion() channel instantiated");

    String version;
    try {
      debugPrint("platformVersion() try");
      final result = await libqaul_channel.invokeMethod('getPlatformVersion');
      debugPrint("platformVersion() result: $result");
      version = 'Android platform version: $result';
    } on PlatformException catch (e) {
      version = "ERROR: libqaul getPlatformVersion: '${e.message}'";
      debugPrint(version);
    }
    return version;
  }

  /// start and initiate libqaul
  Future<void> start() async {
    try {
      await libqaul_channel.invokeMethod('start');
    } on PlatformException catch (e) {
      debugPrint("ERROR: Failed to start libqaul: '${e.message}'");
    }
  }

  /// check if libqaul finished initializing
  ///
  /// returns 1, when qaul finished initializing
  /// otherwise it returns 0
  Future<int> initialized() async {
    int result;
    try {
      if(await libqaul_channel.invokeMethod('initialized')) {
        return 1;
      } else {
        return 0;
      }
    } on PlatformException catch (e) {
      result = -1;
      debugPrint("ERROR: libqaul initialized: '${e.message}'");
    }
    return result;
  }

  /// hello function
  Future<String> hello() async {
    String result;
    try {
      result = await libqaul_channel.invokeMethod('hello');
    } on PlatformException catch (e) {
      result = "hello ERROR";
      debugPrint("ERROR: libqaul hello: '${e.message}'");
    }
    return result;
  }

  /// Debug function: how many rpc messages have been sent to libqaul
  Future<int> checkSendCounter() async {
    int result;
    try {
      result = await libqaul_channel.invokeMethod('sendcounter');
    } on PlatformException catch (e) {
      result = -1;
      debugPrint("ERROR: libqaul sendcounter: '${e.message}'");
    }
    return result;
  }

  /// Debug function: How many rpc messages are queued by libqaul
  Future<int> checkReceiveQueue() async {
    int result;
    try {
      result = await libqaul_channel.invokeMethod('receivequeue');
    } on PlatformException catch (e) {
      result = -1;
      debugPrint("ERROR: libqaul channel receivequeue: '${e.message}'");
    }
    return result;
  }

  /// send binary protobuf RPC message to libqaul
  Future<void> sendRpc(Uint8List message) async {
    try {
      await libqaul_channel.invokeMethod( 'sendRpcMessage', { 'message': message } );
    } on PlatformException catch (e) {
      debugPrint("ERROR: libqaul channel sendRpcMessage: '${e.message}'");
    }
  }

  /// receive binary protobuf RPC message from libqaul
  /// and pass it to RPC module
  Future<void> receiveRpc() async {
    try {
      final Uint8List? result = await libqaul_channel.invokeMethod('receiveRpcMessage');

      if(result == null) {
        print("channel receiveRpcMessage: null received");
      } else {
        print("channel receiveRpcMessage: received");

        if(result.isEmpty) {
          print("channel receiveRpcMessage: result is empty");
          return;
        }

        // check result size
        final size = result.lengthInBytes;
        print("channel receiveRpcMessage: $size bytes received");

        if(size == 0) {
          print("channel receiveRpcMessage: size == 0");
          return;
        }

        // convert the buffer to a list
        final list = result.toList(growable: false);
        print("channel receiveRpcMessage before processing");

        // decode protobuf message
        final rpc = Rpc();
        rpc.decodeReceivedMessage(list);
        print("channel receiveRpcMessage processed");

        // TODO: Free message buffer?
      }
    } on PlatformException catch (e) {
      debugPrint("ERROR: libqaul receiveRpcMessage: '${e.message}'");
    }
  }
}
