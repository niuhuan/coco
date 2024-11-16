import 'package:coco/ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:modal_bottom_sheet/modal_bottom_sheet.dart';
import 'package:photo_view/photo_view.dart';
import '../commons.dart';
import '../image_cache_provider.dart';
import 'components/empty_app_bar.dart';
import 'components/right_click_pop.dart';
import 'components/shadow_icon_button.dart';

class PostInfoScreen extends StatefulWidget {
  final String host;
  final Post post;

  const PostInfoScreen({
    required this.host,
    required this.post,
    Key? key,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _PostInfoScreenState();
}

class _PostInfoScreenState extends State<PostInfoScreen> {
  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: const EmptyAppBar(),
      body: Stack(children: [
        GestureDetector(
          onLongPress: () async {
            String? choose = await chooseListDialog(
              context,
              title: AppLocalizations.of(context)!.pleaseChoose,
              values: [
                AppLocalizations.of(context)!.downloadOriginImage,
              ],
            );
            if (choose == AppLocalizations.of(context)!.downloadOriginImage) {
              _onDownload();
            }
          },
          child: PhotoView.customChild(
            child: PhotoView(
              imageProvider: ImageCacheProvider(
                url: widget.post.sampleUrl,
                useful: 'sample',
                extendsFieldIntFirst: widget.post.id,
                extendsFieldIntSecond: widget.post.parentId,
                extendsFieldIntThird: widget.post.creatorId,
              ),
              loadingBuilder: (context, event) => LayoutBuilder(
                builder: (BuildContext context, BoxConstraints constraints) {
                  return buildLoading(
                    width: constraints.maxWidth,
                    height: constraints.maxHeight,
                    color: Colors.grey.withAlpha(130),
                  );
                },
              ),
            ),
          ),
        ),
        ..._buildNavBar(),
      ]),
    );
  }

  List<Widget> _buildNavBar() {
    return [
      SafeArea(
        child: Align(
          alignment: Alignment.topLeft,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              ShadowIconButton(
                icon: Icons.arrow_back,
                onPressed: () {
                  Navigator.of(context).pop();
                },
              ),
              ShadowIconButton(
                icon: Icons.style,
                onPressed: () async {
                  var popText = await showMaterialModalBottomSheet(
                    context: context,
                    backgroundColor: const Color(0xAA000000),
                    builder: (context) {
                      return SizedBox(
                        height: MediaQuery.of(context).size.height * (.45),
                        child: _TagsSheet(widget.post.tags.split(" ")),
                      );
                    },
                  );
                  if (popText != null && popText is String) {
                    Navigator.of(context).pop(popText);
                  }
                },
              ),
            ],
          ),
        ),
      ),
      SafeArea(
        child: Align(
          alignment: Alignment.topRight,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              ShadowIconButton(
                icon: Icons.download,
                onPressed: _onDownload,
              ),
            ],
          ),
        ),
      ),
    ];
  }

  _onDownload() async {
    bool append = await native.addDownloadPost(
      host: widget.host,
      post: widget.post,
    );
    if (append) {
      defaultToast(
        context,
        AppLocalizations.of(context)!.downloadAppendSuccess,
      );
    } else {
      defaultToast(
        context,
        AppLocalizations.of(context)!.downloadAppendFailed,
      );
    }
  }
}

class _TagsSheet extends StatelessWidget {
  final List<String> tags;

  const _TagsSheet(this.tags);

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: [
        Wrap(
          runSpacing: 3,
          spacing: 3,
          children: [
            for (String tag in tags)
              GestureDetector(
                onTap: () {
                  Navigator.of(context).pop(tag);
                },
                child: Padding(
                  padding: const EdgeInsets.all(1.0),
                  child: Chip(
                    label: Text(tag),
                  ),
                ),
              ),
          ],
        ),
      ],
    );
  }
}
