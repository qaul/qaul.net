// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// initialization routine of qaul app
/// start libqaul

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'libqaul.dart';
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

    // start libqaul
    libqaul.start();
    print("libqaul started");

    // wait some seconds to properly get
    // libqaul up and running
    await Future.delayed(Duration(seconds: 3));

    print("libqaul initialization finished");

    // call hello function
    final hello = libqaul.hello();
    print(hello);

    // request node info
    final rpcNode = RpcNode();
    rpcNode.getNodeInfo();

    // wait a bit
    await Future.delayed(Duration(seconds: 1));

    // DEBUG: how many messages have been sent
    libqaul.checkSendCounter();

    // DEBUG: how many messages are queued by libqaul
    final queued = libqaul.checkReceiveQueue();

    // check for rpc messages
    if(queued > 0) {
      libqaul.receiveRpc();
    }
  }
}