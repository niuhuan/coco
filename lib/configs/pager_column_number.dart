import 'package:coco/ffi.dart';
import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import '../commons.dart';

const _propertyName = "pager_column_number";
late int _pagerColumnNumber;

int get pagerColumnNumber => _pagerColumnNumber;
final pageColumnEvent = Event();

Future initPagerColumnCount() async {
  String numStr = await native.loadProperty(k: _propertyName);
  if (numStr == "") {
    numStr = "2";
  }
  _pagerColumnNumber = int.parse(numStr);
}

Future choosePagerColumnCount(BuildContext context) async {
  final choose = await chooseListDialog(
    context,
    title: AppLocalizations.of(context)!.choosePagerColumnNumber,
    values: List<int>.generate(10, (i) => i + 1),
  );
  if (choose != null) {
    await native.saveProperty(k: _propertyName, v: choose.toString());
    _pagerColumnNumber = choose;
    pageColumnEvent.broadcast();
  }
}
