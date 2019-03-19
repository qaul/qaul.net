import { helper } from '@ember/component/helper';

export function fileIcon([suffix]/*, hash*/) {
  switch (suffix) {
    case 'jpg':
    case 'jpeg':
    case 'png':
    case 'gif':
      return 'image';
    case 'pdf':
      return 'file-pdf'
    case 'mpg4':
    case 'mov':
    case 'mkv':
    case 'mp4v':
      return 'film';
    default:
      return 'file';
  }
}

export default helper(fileIcon);
