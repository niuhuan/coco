import 'package:coco/ffi.dart';
import 'package:coco/screens/components/content_builder.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import 'components/right_click_pop.dart';

class TagChooseScreen extends StatefulWidget {
  final String host;
  final String initTags;

  const TagChooseScreen({
    required this.host,
    required this.initTags,
    Key? key,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => _TagChooseScreenState();
}

class _TagChooseScreenState extends State<TagChooseScreen> {
  late final TextEditingController _editingController = TextEditingController();

  late Future<TagData> _tagDataFuture;
  late Key _tagDataFutureKey;

  void _reset() {
    _tagDataFuture = native.tagSummary(host: widget.host);
    _tagDataFutureKey = UniqueKey();
  }

  @override
  void initState() {
    _editingController.text = widget.initTags;
    _reset();
    super.initState();
  }

  @override
  void dispose() {
    _editingController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return rightClickPop(child: buildScreen(context), context: context);
  }

  Widget buildScreen(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(AppLocalizations.of(context)!.chooseTag),
        actions: [
          _buildNoTags(),
        ],
      ),
      body: _buildBody(),
    );
  }

  Widget _buildNoTags() {
    return IconButton(
      onPressed: () {
        Navigator.of(context).pop("");
      },
      icon: const Icon(Icons.clear),
    );
  }

  Widget _buildBody() {
    return ContentBuilder(
      key: _tagDataFutureKey,
      future: _tagDataFuture,
      onRefresh: () async {
        setState(() {
          _reset();
        });
      },
      successBuilder: (
        BuildContext context,
        AsyncSnapshot<TagData> snapshot,
      ) {
        final data = snapshot.requireData;
        final list = data.tags;
        return Column(children: [
          Container(
            padding: const EdgeInsets.fromLTRB(10, 10, 10, 5),
            child: TextFormField(
              controller: _editingController,
              onChanged: (_) {
                setState(() {});
              },
              decoration: const InputDecoration(
                border: UnderlineInputBorder(),
              ),
            ),
          ),
          Expanded(
            child: Container(
              padding: const EdgeInsets.fromLTRB(10, 5, 10, 10),
              child: _buildList(list),
            ),
          ),
        ]);
      },
    );
  }

  Widget _buildList(List<Tag> list) {
    var text = _editingController.text;
    List<Tag> newList = [];
    for (var value in list) {
      if (text.isEmpty || value.tagNames[0].contains(text)) {
        newList.add(value);
      }
    }
    list = newList;
    return ListView.builder(
      itemCount: list.length,
      itemBuilder: (BuildContext context, int index) {
        final item = list[index];
        return GestureDetector(
          onTap: () {
            Navigator.of(context).pop(item.tagNames[0]);
          },
          child: Container(
            decoration: BoxDecoration(
              border: Border.all(
                color: Theme.of(context).dividerColor,
                width: 0.5,
                style: BorderStyle.solid,
              ),
            ),
            padding: const EdgeInsets.all(10),
            child: Text(item.tagNames[0]),
          ),
        );
      },
    );
  }
}
