// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// root of the qaul app
///
/// starts the app
/// checks initialization & shows initialization screen
/// provides main screen when initialization was finished successfully

import 'package:flutter/material.dart';
import 'package:qaul_app/init.dart';
import 'package:qaul_app/screens/main_screen.dart';
import 'package:qaul_app/screens/start_screen.dart';

void main() {
  runApp(QaulApp());
}

class QaulApp extends StatelessWidget {
  // app initialization state

  final Future _initFuture = Init.initialize();

  // show screen
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'qaul.net – قول ',
      theme: ThemeData(
        brightness: Brightness.light,
        //brightness: Brightness.dark,
        primarySwatch: Colors.lightBlue,
      ),
      debugShowCheckedModeBanner: false,
      home: FutureBuilder(
        future: _initFuture,
        builder: (context, snapshot){
          if (snapshot.connectionState == ConnectionState.done){
            // show main screen when initialization was done
            return MainScreen();
          } else {
            // show start screen during initialization
            return StartScreen();
          }
        },
      ),
    );
  }
}