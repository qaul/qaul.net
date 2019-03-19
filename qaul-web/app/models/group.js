import { hasMany } from 'ember-data/relationships';
import Owner from './user';

export default Owner.extend({
  users: hasMany('user'),
});
