import 'package:coco/commons.dart';
import 'package:coco/ffi.dart';
import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

const hosts = [
  "https://konachan.net",
  "https://konachan.com",
  "https://yande.re",
  "https://rule34.xxx",
];

late String host;
const _k = "host";

Future initHost() async {
  host = await native.loadProperty(k: _k);
  if (host == "") {
    host = hosts[0];
  }
}

final hostEvent = Event();

Future chooseHost(BuildContext context) async {
  final title = AppLocalizations.of(context)!.webSource;
  final net = AppLocalizations.of(context)!.switchToNet;
  final com = AppLocalizations.of(context)!.switchToCom;
  final re = AppLocalizations.of(context)!.switchToRe;
  final xxx = AppLocalizations.of(context)!.switchToXxx;
  final choose = await chooseListDialog(
    context,
    title: title,
    values: [net, com, re, xxx],
  );
  if (choose != null) {
    if (net == choose) {
      host = hosts[0];
    }
    if (com == choose) {
      host = hosts[1];
    }
    if (re == choose) {
      host = hosts[2];
    }
    if (xxx == choose) {
      host = hosts[3];
    }
    await native.saveProperty(k: _k, v: host);
    hostEvent.broadcast();
  }
}
