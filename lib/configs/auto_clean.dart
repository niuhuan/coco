import 'package:coco/ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import '../commons.dart';

const _propertyName = "auto_clean";
late String autoClean;

Map<String, String> _nameMap(BuildContext context) => {
      (3600 * 24).toString(): AppLocalizations.of(context)!.aDay,
      (3600 * 24 * 7).toString(): AppLocalizations.of(context)!.aWeek,
      (3600 * 24 * 30).toString(): AppLocalizations.of(context)!.aMonth,
      (3600 * 24 * 30 * 12).toString(): AppLocalizations.of(context)!.aYear,
    };

Future initAutoClean() async {
  autoClean = await native.loadProperty(k: _propertyName);
  if (autoClean == "") {
    autoClean = "${(3600 * 24 * 7)}";
  }
  await native.autoClean(time: int.parse(autoClean));
}

String autoCleanName(BuildContext context) {
  return _nameMap(context)[autoClean] ?? "-";
}

Future chooseAutoClean(BuildContext context) async {
  String? choose = await chooseMapDialog(context,
      title: AppLocalizations.of(context)!.chooseAutoCleanPeriod,
      values: _nameMap(context).map((key, value) => MapEntry(value, key)));
  if (choose != null) {
    await native.saveProperty(k: _propertyName, v: choose);
    autoClean = choose;
  }
}
