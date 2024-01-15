import 'package:appflowy/mobile/presentation/base/mobile_view_page.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:flutter/material.dart';

class MobileGridScreen extends StatelessWidget {
  static const routeName = '/grid';
  static const viewId = 'id';
  static const viewTitle = 'title';

  const MobileGridScreen({
    super.key,
    required this.id,
    this.title,
  });

  /// view id
  final String id;
  final String? title;

  @override
  Widget build(BuildContext context) {
    return MobileViewPage(
      id: id,
      title: title,
      viewLayout: ViewLayoutPB.Document,
    );
  }
}
