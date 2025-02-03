import 'dart:io';

import 'package:better_open_file/better_open_file.dart';
import 'package:filesize/filesize.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:infinite_scroll_pagination/infinite_scroll_pagination.dart';
import 'package:intl/intl.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:url_launcher/url_launcher.dart';

import '../widgets/widgets.dart';

class FileHistoryScreen extends StatefulHookConsumerWidget {
  const FileHistoryScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<FileHistoryScreen> createState() => _FileHistoryScreenState();
}

class _FileHistoryScreenState extends ConsumerState<FileHistoryScreen> {
  static const _pageSize = 20;

  final _controller = PagingController<int, FileHistoryEntity>(firstPageKey: 0);

  @override
  void initState() {
    super.initState();
    _controller.addPageRequestListener((pageKey) => _fetchPage(pageKey));
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return ResponsiveScaffold(
      icon: Icons.history,
      title: l10n.fileHistory,
      body: PagedListView<int, FileHistoryEntity>(
        pagingController: _controller,
        builderDelegate: PagedChildBuilderDelegate<FileHistoryEntity>(
          noItemsFoundIndicatorBuilder: (_) => Text(l10n.noneAvailableMessage),
          itemBuilder: (context, file, index) {
            return Padding(
              padding: const EdgeInsets.all(4.0),
              child: _FileHistoryTile(file: file),
            );
          },
        ),
      ),
    );
  }

  Future<void> _fetchPage(int page) async {
    try {
      final items = await ref
          .read(qaulWorkerProvider)
          .getFileHistory(page: page, itemsPerPage: _pageSize);

      final isLastPage = items.length < _pageSize;
      if (isLastPage) {
        _controller.appendLastPage(items);
      } else {
        final nextPageKey = page + items.length;
        _controller.appendPage(items, nextPageKey);
      }
    } catch (error) {
      _controller.error = error;
    }
  }
}

class _FileHistoryTile extends ConsumerWidget {
  const _FileHistoryTile({Key? key, required this.file}) : super(key: key);
  final FileHistoryEntity file;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final theme = Theme.of(context).textTheme;
    return ListTile(
      onTap: () => _openFile(file, ref),
      leading: _getLeading(ref),
      isThreeLine: true,
      subtitle: _getContent(ref, file: file, theme: theme),
    );
  }

  Widget _getLeading(WidgetRef ref) {
    DecorationImage? image;

    const imageExts = ['gif', 'png', 'jpg', 'jpeg'];
    if (imageExts.contains(file.extension)) {
      final img = File.fromUri(Uri.file(file.filePath(ref)));
      if (img.existsSync()) {
        image = DecorationImage(
          fit: BoxFit.cover,
          image: FileImage(img),
        );
      }
    }

    return Container(
      width: 80,
      height: double.maxFinite,
      alignment: Alignment.center,
      decoration: BoxDecoration(
        color: Colors.black26,
        borderRadius: BorderRadius.circular(8),
        image: image,
      ),
      child: image == null
          ? FaIcon(_getIconFrom(extension: file.extension))
          : const SizedBox(),
    );
  }

  void _openFile(FileHistoryEntity file, WidgetRef ref) async {
    if (Platform.isIOS || Platform.isAndroid) {
      OpenFile.open(file.filePath(ref));
      return;
    }

    final fileUri = Uri.file(file.filePath(ref));

    final parentDirectory = File.fromUri(fileUri).parent.uri;

    for (final uri in [fileUri, parentDirectory]) {
      if (await canLaunchUrl(uri)) {
        launchUrl(uri);
        return;
      }
    }
  }

  Widget _getContent(
    WidgetRef ref, {
    required FileHistoryEntity file,
    required TextTheme theme,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(file.name, style: theme.titleSmall),
        const SizedBox(height: 4),
        Text(file.description),
        const SizedBox(height: 8),
        Text(
          '${DateFormat('EEEE, MMMM d yyyy, h:mm a').format(file.time)} · Size: ${filesize(file.size)}',
          style: const TextStyle(fontStyle: FontStyle.italic),
        ),
      ],
    );
  }

  IconData _getIconFrom({required String extension}) {
    switch (extension) {
      case '7z':
      case 'gz':
      case 'deb':
      case 'pkg':
      case 'rar':
      case 'rpm':
      case 'zip':
      case 'bz2':
      case 'tar':
        return FontAwesomeIcons.solidFileZipper;

      case 'sh':
      case 'md':
      case 'rs':
      case 'arb':
      case 'css':
      case 'yml':
      case 'dart':
      case 'json':
      case 'html':
      case 'yaml':
      case 'toml':
      case 'lock':
        return FontAwesomeIcons.solidFileCode;

      case 'odt':
      case 'rtf':
      case 'txt':
      case 'doc':
      case 'docx':
        return FontAwesomeIcons.solidFileLines;

      case 'bmp':
      case 'gif':
      case 'eps':
      case 'raw':
      case 'png':
      case 'jpg':
      case 'jpeg':
      case 'tif':
      case 'tiff':
        return FontAwesomeIcons.solidFileImage;

      case 'mp3':
      case 'wav':
        return FontAwesomeIcons.solidFileAudio;

      case 'mp4':
      case 'mov':
      case 'wmv':
      case 'mkv':
      case 'avi':
        return FontAwesomeIcons.solidFileVideo;

      case 'pdf':
        return FontAwesomeIcons.solidFilePdf;

      default:
        return FontAwesomeIcons.solidFile;
    }
  }
}
