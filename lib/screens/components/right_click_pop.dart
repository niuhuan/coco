import 'package:flutter/material.dart';

Widget rightClickPop({
  required Widget child,
  required BuildContext context,
  bool canPop = true,
}) =>
    GestureDetector(
      onSecondaryTap: () {
        if (canPop) {
          Navigator.of(context).pop();
        }
      },
      child: child,
    );
