import 'dart:io';

import 'package:coco/configs/auto_clean.dart';
import 'package:coco/configs/host.dart';
import 'package:coco/configs/pager_column_number.dart';
import 'package:coco/configs/versions.dart';
import 'package:coco/ffi.dart';
import 'package:coco/screens/posts_screen.dart';
import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import '../commons.dart';
import '../configs/pager_controller_mode.dart';
import '../configs/platform.dart';
import '../configs/proxy.dart';
import '../cross.dart';

class InitScreen extends StatefulWidget {
  const InitScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _InitScreenState();
}

class _InitScreenState extends State<InitScreen> {
  Future _init() async {
    await initPlatform();
    if (Platform.isAndroid) {
      late bool g;
      if (androidVersion < 30) {
        g = await Permission.storage.request().isGranted;
      } else {
        g = await Permission.manageExternalStorage.request().isGranted;
      }
      if (!g) {
        defaultToast(
          context,
          AppLocalizations.of(context)!.permissionDenied,
        );
        exit(0);
      }
    }
    await native.init(
      root: await cross.root(),
      downloadsTo: await cross.downloads(),
    );
    await initHost();
    await initPagerColumnCount();
    await initPagerControllerMode();
    await initVersion();
    await initAutoClean();
    await initProxy();
    autoCheckNewVersion();
    native.resetFailedDownloads(); // async
    Navigator.of(context).pushReplacement(MaterialPageRoute(
      builder: (BuildContext context) {
        return const PostsScreen();
      },
    ));
  }

  @override
  void initState() {
    _init();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xffbbb6a4),
      body: ConstrainedBox(
        constraints: const BoxConstraints.expand(),
        child: LayoutBuilder(
          builder: (BuildContext context, BoxConstraints constraints) {
            var min = constraints.maxWidth > constraints.maxHeight
                ? constraints.maxHeight
                : constraints.maxWidth;
            var padding = min / 6;
            return Container(
              padding: EdgeInsets.all(padding),
              child: Image.asset(
                "lib/assets/startup.png",
                fit: BoxFit.contain,
              ),
            );
          },
        ),
      ),
    );
  }
}
