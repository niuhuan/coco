import 'package:coco/ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import '../commons.dart';

const _propertyName = "proxy";
late String proxy;

Future initProxy() async {
  proxy = await native.loadProperty(k: _propertyName);
  try {
    native.setProxy(url: proxy);
  } catch (e, s) {
    print("$e\n$s");
  }
}

Future inputProxy(BuildContext context) async {
  String? input = await displayTextInputDialog(
    context,
    title: AppLocalizations.of(context)!.inputProxy,
    hint: "socks5://127.0.0.1:1080/",
    src: proxy,
  );
  if (input != null) {
    try {
      await native.setProxy(url: input);
      await native.saveProperty(k: _propertyName, v: input);
      proxy = input;
    } catch (e, s) {
      print("$e\n$s");
      defaultToast(context, "$e");
    }
  }
}
