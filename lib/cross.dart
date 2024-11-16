import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import 'commons.dart';
import 'ffi.dart';

const cross = Cross._();

class Cross {
  const Cross._();

  static const _channel = MethodChannel("cross");

  Future<String> downloads() async {
    if (Platform.isIOS) {
      return await _channel.invokeMethod("downloads_to");
    } else if (Platform.isAndroid) {
      return await _channel.invokeMethod("downloads_to");
    } else if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
      return await native.downloadsTo();
    }
    throw "没有适配的平台";
  }

  Future<String> root() async {
    if (Platform.isAndroid || Platform.isIOS) {
      return await _channel.invokeMethod("root");
    }
    if (Platform.isWindows || Platform.isMacOS || Platform.isLinux) {
      return await native.desktopRoot();
    }
    throw "没有适配的平台";
  }

  Future<int> androidGetVersion() async {
    return await _channel.invokeMethod("androidGetVersion");
  }
//
// Future saveImageToGallery(String path, BuildContext context) async {
//   if (Platform.isAndroid) {
//     if (!(await Permission.storage.request()).isGranted) {
//       return;
//     }
//   }
//   if (Platform.isIOS || Platform.isAndroid) {
//     try {
//       await _channel.invokeMethod("saveImageToGallery", path);
//       defaultToast(
//         context,
//         AppLocalizations.of(context)!.success,
//       );
//     } catch (e) {
//       // todo errorToast
//       defaultToast(
//         context,
//         "${AppLocalizations.of(context)!.failed} : $e",
//       );
//     }
//   } else if (Platform.isWindows || Platform.isMacOS || Platform.isLinux) {
//     String? selectedDirectory = await FilePicker.platform.getDirectoryPath();
//     if (selectedDirectory != null) {
//       try {
//         await native.copyImageTo(srcPath: path, toDir: selectedDirectory);
//         defaultToast(
//           context,
//           AppLocalizations.of(context)!.success,
//         );
//       } catch (e) {
//         // todo errorToast
//         defaultToast(
//           context,
//           "${AppLocalizations.of(context)!.failed} : $e",
//         );
//       }
//     }
//   }
// }
}

/// 打开web页面
Future<dynamic> openUrl(String url) async {
  if (await canLaunch(url)) {
    await launch(
      url,
      forceSafariVC: false,
    );
  }
}
