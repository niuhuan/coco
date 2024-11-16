import 'package:coco/screens/post_info_screen.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:waterfall_flow/waterfall_flow.dart';

import '../../configs/pager_column_number.dart';
import '../../ffi.dart';
import '../../image_cache_provider.dart';

class PostPagerListener {
  final Function(String) onTag;

  const PostPagerListener({
    required this.onTag,
  });
}

class PostPager extends StatefulWidget {
  final String host;
  final String tags;
  final PostPagerListener listener;

  PostPager({required this.host, required this.tags, required this.listener})
      : super(key: Key("$host|$tags"));

  @override
  State<StatefulWidget> createState() => _PostPagerState();
}

class _PostPagerState extends State<PostPager> {
  final List<Post> _list = [];
  int _page = 1;

  // int _maxPage = 0;
  bool _loading = false;
  bool _fail = false;
  bool _over = false;

  final _controller = ScrollController();

  Future _loadNextPage() async {
    if (_over) return;
    setState(() {
      _fail = false;
      _loading = true;
    });
    try {
      var list = await native.loadPosts(
        host: widget.host,
        tags: widget.tags,
        page: _page,
      );
      if (list.posts.isEmpty) {
        _over = true;
      }
      _list.addAll(list.posts);
      _page++;
      // 会造成连续加载
      // if (_over == false &&
      //     _fail == false &&
      //     _controller.position.maxScrollExtent == 0) {
      //   // 异步加载下一页
      //   var _ = _loadNextPage();
      // }
    } catch (e, s) {
      print("$e");
      print("$s");
      _fail = true;
    }
    setState(() {
      _loading = false;
    });
  }

  void _onScroll() {
    if (_loading) {
      return;
    }
    if (_controller.position.pixels + 100 >
        _controller.position.maxScrollExtent) {
      _loadNextPage();
    }
  }

  @override
  void initState() {
    _loadNextPage();
    pageColumnEvent.subscribe(_setState);
    super.initState();
  }

  @override
  void dispose() {
    pageColumnEvent.unsubscribe(_setState);
    _controller.dispose();
    super.dispose();
  }

  _setState(_) {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return _buildFlow();
  }

  Widget _buildFlow() {
    var listView = WaterfallFlow.builder(
      controller: _controller,
      physics: const AlwaysScrollableScrollPhysics(),
      padding: const EdgeInsets.only(top: 10, bottom: 10),
      itemCount: _list.length + 1,
      gridDelegate: SliverWaterfallFlowDelegateWithFixedCrossAxisCount(
        crossAxisCount: pagerColumnNumber,
        crossAxisSpacing: 0,
        mainAxisSpacing: 0,
      ),
      itemBuilder: (BuildContext context, int index) {
        if (index >= _list.length) {
          return _buildLoadingCard();
        }
        return _buildImageCard(_list[index]);
      },
    );
    return NotificationListener(
      child: listView,
      onNotification: (scrollNotification) {
        _onScroll();
        return true;
      },
    );
  }

  Widget _buildLoadingCard() {
    if (_loading) {
      return Card(
        child: Column(
          children: [
            Container(
              padding: const EdgeInsets.only(top: 10, bottom: 10),
              child: const CupertinoActivityIndicator(
                radius: 14,
              ),
            ),
            Text(AppLocalizations.of(context)!.loading),
          ],
        ),
      );
    }

    if (_fail) {
      return Card(
        child: InkWell(
          onTap: () {
            _loadNextPage();
          },
          child: Column(
            children: [
              Container(
                padding: const EdgeInsets.only(top: 10, bottom: 10),
                child: const Icon(Icons.sync_problem_rounded),
              ),
              Text(AppLocalizations.of(context)!.exceptionTapToRetry),
            ],
          ),
        ),
      );
    }

    // todo if over
    return Container();
  }

  Widget _buildImageCard(Post item) {
    return GestureDetector(
      onTap: () async {
        var popText = await Navigator.of(context).push(MaterialPageRoute(
          builder: (context) {
            return PostInfoScreen(host: widget.host, post: item);
          },
        ));
        if (popText != null && popText is String) {
          widget.listener.onTag(popText);
        }
      },
      child: Card(
        child: LayoutBuilder(
          builder: (BuildContext context, BoxConstraints constraints) {
            return Image(
              fit: BoxFit.contain,
              width: constraints.maxWidth,
              height: constraints.maxWidth * item.height / item.width,
              image: ImageCacheProvider(
                url: item.previewUrl,
                useful: 'preview',
                extendsFieldIntFirst: item.id,
                extendsFieldIntSecond: item.parentId,
                extendsFieldIntThird: item.creatorId,
              ),
            );
          },
        ),
      ),
    );
  }
}
