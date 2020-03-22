import { attr, hasMany } from '@ember-data/model';
import Owner from './user';

export default Owner.extend({
  multiuser: attr('boolean'), // a group with more than 2 users
  name: attr('string'),
  avatar: attr('string'),
  users: hasMany('user'),
  messages: hasMany('chatmessage', { inverse: 'group' }),
});
