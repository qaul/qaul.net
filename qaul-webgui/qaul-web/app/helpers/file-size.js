import { helper } from '@ember/component/helper';

const YiB = Math.pow(2,80);
const ZiB = Math.pow(2,70);
const EiB = Math.pow(2,60);
const PiB = Math.pow(2,50);
const TiB = Math.pow(2,40);
const GiB = Math.pow(2,30);
const MiB = Math.pow(2,20);
const KiB = Math.pow(2,10);

export function fileSize([size]/*, hash*/) {
  if(typeof size === 'string') {
    size = Number.parseInt(size);
  }

  if (size >= YiB){
    return `${Math.floor(size / YiB)}YiB`
  }
  if (size >= ZiB){
    return `${Math.floor(size / ZiB)}ZiB`
  }
  if (size >= EiB){
    return `${Math.floor(size / EiB)}EiB`
  }
  if (size >= PiB){
    return `${Math.floor(size / PiB)}PiB`
  }
  if (size >= TiB){
    return `${Math.floor(size / TiB)}TiB`
  }
  if (size >= GiB){
    return `${Math.floor(size / GiB)}GiB`
  }
  if (size >= MiB){
    return `${Math.floor(size / MiB)}MiB`
  }
  if (size >= KiB){
    return `${Math.floor(size / KiB)}KiB`
  }

  return `${size}B`;
}

export default helper(fileSize);
