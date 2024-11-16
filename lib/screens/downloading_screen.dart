import 'package:coco/bridge_generated.dart';
import 'package:coco/ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:photo_view/photo_view.dart';
import 'package:share_plus/share_plus.dart';
import 'package:waterfall_flow/waterfall_flow.dart';

import '../commons.dart';
import '../cross.dart';
import '../image_cache_provider.dart';
import 'components/content_builder.dart';
import 'components/empty_app_bar.dart';
import 'components/right_click_pop.dart';
import 'components/shadow_icon_button.dart';

class DownloadingScreen extends StatefulWidget {
  const DownloadingScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _DownloadingScreenState();
}

class _DownloadingScreenState extends State<DownloadingScreen> {
  Future<List<DlPost>> _future = native.allDownloads();
  Key _key = UniqueKey();

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(AppLocalizations.of(context)!.downloading),
        actions: [
          _resetAction(),
        ],
      ),
      body: ContentBuilder(
        key: _key,
        future: _future,
        onRefresh: () async {},
        successBuilder: (
          BuildContext context,
          AsyncSnapshot<List<DlPost>> snapshot,
        ) {
          final _list = snapshot.requireData;
          return WaterfallFlow.builder(
            physics: const AlwaysScrollableScrollPhysics(),
            padding: const EdgeInsets.only(top: 10, bottom: 10),
            itemCount: _list.length,
            gridDelegate:
                const SliverWaterfallFlowDelegateWithFixedCrossAxisCount(
              crossAxisCount: 2,
              crossAxisSpacing: 0,
              mainAxisSpacing: 0,
            ),
            itemBuilder: (BuildContext context, int index) {
              return _buildImageCard(_list[index]);
            },
          );
        },
      ),
    );
  }

  Widget _buildImageCard(DlPost item) {
    return GestureDetector(
      // onTap: () {
      //   if (item.dlStatus == 1) {
      //     Navigator.of(context).push(MaterialPageRoute(
      //       builder: (context) {
      //         return DlPostScreen(item);
      //       },
      //     ));
      //   }
      // },
      child: Card(
        child: LayoutBuilder(
          builder: (BuildContext context, BoxConstraints constraints) {
            final width = constraints.maxWidth;
            final height = constraints.maxWidth * item.height / item.width;
            final info = Stack(
              children: [
                Image(
                  fit: BoxFit.contain,
                  width: width,
                  height: height,
                  image: ImageCacheProvider(
                    url: item.previewUrl,
                    useful: 'preview',
                    extendsFieldIntFirst: item.id,
                    extendsFieldIntSecond: item.parentId,
                    extendsFieldIntThird: item.creatorId,
                  ),
                ),
                // dlStatus
                SizedBox(
                  width: width,
                  height: height,
                  child: Align(
                    alignment: Alignment.topRight,
                    child: Container(
                      margin: const EdgeInsets.all(3),
                      padding: const EdgeInsets.all(2),
                      decoration: BoxDecoration(
                        color: (item.dlStatus == 0
                                ? Colors.orange
                                : item.dlStatus == 1
                                    ? Colors.green
                                    : Colors.red)
                            .withOpacity(.8),
                        borderRadius: BorderRadius.circular(1),
                      ),
                      child: Icon(
                        item.dlStatus == 0
                            ? Icons.download_outlined
                            : item.dlStatus == 1
                                ? Icons.download
                                : item.dlStatus == 2
                                    ? Icons.dangerous
                                    : item.dlStatus == 3
                                        ? Icons.delete
                                        : Icons.question_mark,
                        color: Colors.white,
                        size: 10,
                      ),
                    ),
                  ),
                ),
              ],
            );
            return GestureDetector(
              onLongPress: () async {
                String? choose = await chooseListDialog(
                  context,
                  title: AppLocalizations.of(context)!.pleaseChoose,
                  values: [
                    AppLocalizations.of(context)!.delete,
                  ],
                );
                if (choose ==
                    AppLocalizations.of(context)!.delete) {
                  await native.deleteDlPost(dlKey: item.dlKey);
                  // mark item.dlStatus = 3, but can't modify final;
                  setState(() {
                    _future = native.allDownloads();
                    _key = UniqueKey();
                  });
                  defaultToast(
                    context,
                    AppLocalizations.of(context)!.success,
                  );
                }
              },
              child: info,
            );
          },
        ),
      ),
    );
  }

  Widget _resetAction() {
    return IconButton(
      onPressed: () async {
        await native.resetFailedDownloads();
        setState(() {
          _future = native.allDownloads();
          _key = UniqueKey();
        });
        defaultToast(
          context,
          AppLocalizations.of(context)!.resetFailedDownloadsAndReload,
        );
      },
      icon: const Icon(Icons.refresh_sharp),
    );
  }
}

// class DlPostScreen extends StatefulWidget {
//   final DlPost item;
//
//   const DlPostScreen(this.item, {Key? key}) : super(key: key);
//
//   @override
//   State<StatefulWidget> createState() => _DlPostScreenState();
// }
//
// class _DlPostScreenState extends State<DlPostScreen> {
//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       backgroundColor: Colors.black,
//       appBar: const EmptyAppBar(),
//       body: Stack(children: [
//         PhotoView.customChild(
//           child: PhotoView(
//             imageProvider: DownloadImageProvider(
//               dlKey: widget.item.dlKey,
//             ),
//             loadingBuilder: (context, event) => LayoutBuilder(
//               builder: (BuildContext context, BoxConstraints constraints) {
//                 return buildLoading(
//                   width: constraints.maxWidth,
//                   height: constraints.maxHeight,
//                   color: Colors.grey.withAlpha(130),
//                 );
//               },
//             ),
//           ),
//         ),
//         ..._buildNavBar(),
//       ]),
//     );
//   }
//
//   List<Widget> _buildNavBar() {
//     return [
//       SafeArea(
//         child: Align(
//           alignment: Alignment.topLeft,
//           child: ShadowIconButton(
//               icon: Icons.arrow_back,
//               onPressed: () {
//                 Navigator.of(context).pop();
//               }),
//         ),
//       ),
//       SafeArea(
//         child: Align(
//           alignment: Alignment.topRight,
//           child: Row(
//             mainAxisAlignment: MainAxisAlignment.end,
//             children: [
//               ShadowIconButton(icon: Icons.share, onPressed: _onShare),
//               ShadowIconButton(
//                 icon: Icons.save_rounded,
//                 onPressed: _onSave,
//               ),
//             ],
//           ),
//         ),
//       ),
//     ];
//   }
//
//   _onShare() async {
//     Share.shareFiles([
//       await native.loadDlImage(dlKey: widget.item.dlKey),
//     ]);
//   }
//
//   _onSave() async {
//     await cross.saveImageToGallery(
//       await native.loadDlImage(dlKey: widget.item.dlKey),
//       context,
//     );
//   }
// }
