import 'package:coco/configs/host.dart';
import 'package:coco/screens/about_screen.dart';
import 'package:coco/screens/components/badge.dart';
import 'package:coco/screens/components/post_pager.dart';
import 'package:coco/screens/tag_choose_screen.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_styled_toast/flutter_styled_toast.dart';

import 'components/browser_bottom_sheet.dart';
import 'downloading_screen.dart';

class PostsScreen extends StatefulWidget {
  const PostsScreen({
    Key? key,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _PostsScreenState();
}

class _PostsScreenState extends State<PostsScreen> {
  String _tags = "";

  @override
  void initState() {
    hostEvent.subscribe(_setState);
    super.initState();
  }

  @override
  void dispose() {
    hostEvent.unsubscribe(_setState);
    super.dispose();
  }

  _setState(_) {
    setState(() {});
  }

  var _noticeTime = 0;

  @override
  Widget build(BuildContext context) {
    return WillPopScope(
      child: buildScreen(context),
      onWillPop: () async {
        final now = DateTime.now().millisecondsSinceEpoch;
        if (_noticeTime + 3000 > now) {
          return true;
        } else {
          _noticeTime = now;
          showToast(
            AppLocalizations.of(context)!.pressBackKeyAgainToExit,
            context: context,
            position: StyledToastPosition.center,
            animation: StyledToastAnimation.scale,
            reverseAnimation: StyledToastAnimation.fade,
            duration: const Duration(seconds: 3),
            animDuration: const Duration(milliseconds: 300),
            curve: Curves.elasticOut,
            reverseCurve: Curves.linear,
          );
          return false;
        }
        return true;
      },
    );
  }

  Widget buildScreen(BuildContext context) {
    final tags = _tags == "" ? AppLocalizations.of(context)!.all : _tags;
    return Scaffold(
      appBar: AppBar(
        title: Text(tags),
        actions: [
          IconButton(
              onPressed: () {
                Navigator.of(context).push(
                  MaterialPageRoute(builder: (BuildContext context) {
                    return const AboutScreen();
                  }),
                );
              },
              icon: const VersionBadged(
                child: Icon(Icons.info_outline),
              )),
          const BrowserBottomSheetAction(),
          _downloadsAction(),
          _buildTagAction(),
        ],
      ),
      body: PostPager(
        host: host,
        tags: _tags,
        listener: PostPagerListener(onTag: _onTag),
      ),
    );
  }

  Widget _buildTagAction() {
    return IconButton(
      onPressed: () async {
        String? choose = await Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return TagChooseScreen(host: host, initTags: _tags);
          },
        ));
        if (choose != null) {
          _onTag(choose);
        }
      },
      icon: const Icon(Icons.style),
    );
  }

  Widget _downloadsAction() {
    return IconButton(
      onPressed: () async {
        await Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return const DownloadingScreen();
          },
        ));
      },
      icon: const Icon(Icons.download),
    );
  }

  void _onTag(String choose) {
    setState(() {
      _tags = choose;
    });
  }
}
