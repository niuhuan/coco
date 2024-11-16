import 'package:coco/ffi.dart';
import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import '../commons.dart';

const _propertyKey = "pager_controller_mode";
late PagerControllerMode _value;
final currentPagerControllerModeEvent = Event();

PagerControllerMode get currentPagerControllerMode => _value;

enum PagerControllerMode {
  stream,
  pager,
}

Map<PagerControllerMode, String> _nameMap(BuildContext context) => {
      PagerControllerMode.stream: AppLocalizations.of(context)!.stream,
      PagerControllerMode.pager: AppLocalizations.of(context)!.pager,
    };

String currentPagerControllerModeName(BuildContext context) =>
    _nameMap(context)[_value]!;

Future choosePagerControllerMode(BuildContext context) async {
  final target = await chooseMapDialog(context,
      title: AppLocalizations.of(context)!.choosePagerMode,
      values: _nameMap(context).map((key, value) => MapEntry(value, key)));
  if (target != null && target != _value) {
    await native.saveProperty(k: _propertyKey, v: "$target");
    _value = target;
    currentPagerControllerModeEvent.broadcast();
  }
}

PagerControllerMode _parse(String string) {
  for (var value in PagerControllerMode.values) {
    if ("$value" == string) {
      return value;
    }
  }
  return PagerControllerMode.stream;
}

Future initPagerControllerMode() async {
  _value = _parse(await native.loadProperty(k: _propertyKey));
}
