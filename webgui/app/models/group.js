import { hasMany } from '@ember-data/model';
import Owner from './user';

export default Owner.extend({
  users: hasMany('user'),
});
