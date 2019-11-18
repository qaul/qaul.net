import DS from 'ember-data';
import { computed, get } from '@ember/object';
import moment from 'moment';
// import Owner from './user';

//export default Owner.extend({
export default DS.Model.extend({
    fpToken: DS.attr('string'),
    username: DS.attr('string'),
    bio: DS.attr('string'),
    trust: DS.attr('number'),
    starred: DS.attr('boolean'),
    // age or birthdate?
    // avatar and token link?
    identicon: DS.attr('string'),
    avatar: DS.attr('string'),

    gender: DS.attr('string'),

    lastSeen: DS.attr('date'),

    online: computed('lastSeen', {
        get() {
            return moment.duration(moment().diff(get(this, 'lastSeen'))).asMinutes() < 2;
        }
    }),
});
