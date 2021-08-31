// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/// first screen which is shown during startup and initialization of qaul

import 'package:flutter/material.dart';

class StartScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Material(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          Text(
            " qaul – قول ",
            style: TextStyle(
              fontSize: 24,
            ),
          ),
          SizedBox(height: 40),
          CircularProgressIndicator()
        ],
      ),
    );
  }
}