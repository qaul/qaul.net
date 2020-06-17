import Model, { attr } from '@ember-data/model';
// import { computed, get } from '@ember/object';
// import moment from 'moment';
// import Owner from './user';

//export default Owner.extend({
export default Model.extend({
  displayName: attr('string'),
  realName: attr('string'),

  // fpToken: attr('string'),
  // username: attr('string'),
  // bio: attr('string'),
  trust: attr('number'),
  // starred: attr('boolean'),
  // // age or birthdate?
  // // avatar and token link?
  // identicon: attr('string'),
  avatar: attr('string'),

  // gender: attr('string'),

  lastSeen: attr('date'),

  online: attr('boolean'),
  // online: computed('lastSeen', {
  //     get() {
  //         return moment.duration(moment().diff(get(this, 'lastSeen'))).asMinutes() < 2;
  //     }
  // }),
});
