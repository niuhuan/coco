import 'package:coco/configs/proxy.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:modal_bottom_sheet/modal_bottom_sheet.dart';
import 'package:flutter/material.dart';

import '../../commons.dart';
import '../../configs/auto_clean.dart';
import '../../configs/host.dart';
import '../../configs/pager_column_number.dart';
import '../../configs/pager_controller_mode.dart';
import '../../ffi.dart';

class BrowserBottomSheetAction extends StatelessWidget {
  const BrowserBottomSheetAction({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: () {
        _displayBrowserBottomSheet(context);
      },
      icon: const Icon(Icons.menu),
    );
  }
}

Future _displayBrowserBottomSheet(BuildContext context) async {
  await showMaterialModalBottomSheet(
    context: context,
    backgroundColor: const Color(0xAA000000),
    builder: (context) {
      return SizedBox(
        height: MediaQuery.of(context).size.height * (.45),
        child: _BrowserBottomSheet(),
      );
    },
  );
}

class _BrowserBottomSheet extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => _BrowserBottomSheetState();
}

class _BrowserBottomSheetState extends State<_BrowserBottomSheet> {

  @override
  Widget build(BuildContext context) {
    var hostName = "";
    if (host == hosts[0]) {
      hostName = ".NET";
    }
    else if (host == hosts[1]) {
      hostName = ".COM";
    }
    else if (host == hosts[2]) {
      hostName = ".RE";
    }
    else if (host == hosts[3]) {
      hostName = ".XXX";
    }
    return ListView(
      children: [
        Row(
          children: [
            Expanded(child: Container()),
            _bottomIcon(
              icon: Icons.repeat_one,
              title: hostName,
              onPressed: () async {
                await chooseHost(context);
                setState(() {});
              },
            ),
            Expanded(child: Container()),
            _bottomIcon(
              icon: Icons.view_day_outlined,
              title: currentPagerControllerModeName(context),
              onPressed: () async {
                await choosePagerControllerMode(context);
                setState(() {});
              },
            ),
            Expanded(child: Container()),
            _bottomIcon(
              icon: Icons.view_column_sharp,
              title: "$pagerColumnNumber",
              onPressed: () async {
                await choosePagerColumnCount(context);
                setState(() {});
              },
            ),
            Expanded(child: Container()),
          ],
        ),
        Row(
          children: [
            Expanded(child: Container()),
            _bottomIcon(
              icon: Icons.network_ping,
              title: "proxy",
              onPressed: () async {
                await inputProxy(context);
                setState(() {});
              },
            ),
            Expanded(child: Container()),
            _bottomIcon(
              icon: Icons.cleaning_services_rounded,
              title: AppLocalizations.of(context)!.clean,
              onPressed: () async {
                defaultToast(context, AppLocalizations.of(context)!.cleaning);
                try {
                  await native.cleanAllCache();
                  defaultToast(context, AppLocalizations.of(context)!.cleanSuccess);
                } catch (e) {
                  print("$e");
                  defaultToast(context, AppLocalizations.of(context)!.cleanFailed);
                }
                setState(() {});
              },
            ),
            Expanded(child: Container()),
            _bottomIcon(
              icon: Icons.auto_delete_outlined,
              title: autoCleanName(context),
              onPressed: () async {
                await chooseAutoClean(context);
                setState(() {});
              },
            ),
            Expanded(child: Container()),
          ],
        ),
      ],
    );
  }

  Widget _bottomIcon({
    required IconData icon,
    required String title,
    required void Function() onPressed,
  }) {
    return Expanded(
      child: Center(
        child: Column(
          children: [
            IconButton(
              iconSize: 55,
              icon: Column(
                children: [
                  Container(height: 3),
                  Icon(
                    icon,
                    size: 25,
                    color: Colors.white,
                  ),
                  Container(height: 3),
                  Text(
                    title,
                    style: const TextStyle(color: Colors.white, fontSize: 10),
                    maxLines: 1,
                    textAlign: TextAlign.center,
                  ),
                  Container(height: 3),
                ],
              ),
              onPressed: onPressed,
            )
          ],
        ),
      ),
    );
  }
}
