// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// initialization routine of qaul app
/// start libqaul

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'libqaul/libqaul.dart';
import '/rpc/protobuf.dart';
import '/rpc/node.dart';

class Init {
  static final container = ProviderContainer();

  static Future<void> initialize() async {
    print("initialize libqaul");
    // load libqaul
    // get it from provider
    final libqaul = container.read(libqaulProvider);
    print("libqaul loaded");

    // test platform function
    final platform = await libqaul.getPlatformVersion();
    print(platform);

    // call hello function
    final hello = await libqaul.hello();
    print(hello);

    // start libqaul
    await libqaul.start();
    print("libqaul started");

    // check if libqaul finished initializing
    //await Future.delayed(Duration(seconds: 3));
    while (libqaul.initialized() == 0) {
      await Future.delayed(Duration(milliseconds: 10));
    }

    print("libqaul initialization finished");

    // request node info
    final rpcNode = RpcNode();
    await rpcNode.getNodeInfo();

    // wait a bit
    await Future.delayed(Duration(seconds: 1));

    // DEBUG: how many messages have been sent
    final sent = await libqaul.checkSendCounter();
    print("libqaul checkSendCounter: $sent");

    // DEBUG: how many messages are queued by libqaul
    final queued = await libqaul.checkReceiveQueue();
    print("libqaul checkReceiveQueue: $queued");

    // check for rpc messages
    if(queued > 0) {
      print("libqaul receiveRpc");
      final rpc_received = await libqaul.receiveRpc();
      print("libqaul RPC receveid");
    }
  }
}