import 'dart:async' show Future;
import 'dart:convert';
import 'package:event/event.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart' show rootBundle;
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import '../commons.dart';
import '../ffi.dart';


const _versionUrl =
    "https://api.github.com/repos/niuhuan/coco/releases/latest";
const _versionAssets = 'lib/assets/version.txt';
RegExp _versionExp = RegExp(r"^v\d+\.\d+.\d+$");

late String _version;
String? _latestVersion;
String? _latestVersionInfo;

Future initVersion() async {
  // 当前版本
  try {
    _version = (await rootBundle.loadString(_versionAssets)).trim();
  } catch (e) {
    _version = "dirty";
  }
}

var versionEvent = Event<EventArgs>();

String currentVersion() {
  return _version;
}

String? get latestVersion => _latestVersion;

String? latestVersionInfo() {
  return _latestVersionInfo;
}

Future autoCheckNewVersion() {
  return _versionCheck();
}

Future manualCheckNewVersion(BuildContext context) async {
  try {
    defaultToast(context, AppLocalizations.of(context)!.checkingNewVersion);
    await _versionCheck();
    defaultToast(context, AppLocalizations.of(context)!.success);
  } catch (e) {
    defaultToast(context, AppLocalizations.of(context)!.failed + " : $e");
  }
}

bool dirtyVersion() {
  return !_versionExp.hasMatch(_version);
}

// maybe exception
Future _versionCheck() async {
  if (_versionExp.hasMatch(_version)) {
    var json = jsonDecode(await native.httpGet(url: _versionUrl));
    if (json["name"] != null) {
      String latestVersion = (json["name"]);
      if (latestVersion != _version) {
        _latestVersion = latestVersion;
        _latestVersionInfo = json["body"] ?? "";
      }
    }
  } // else dirtyVersion
  versionEvent.broadcast();
  print("$_latestVersion");
}
