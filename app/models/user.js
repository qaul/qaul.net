import DS from 'ember-data';
import { computed, get } from '@ember/object';
import moment from 'moment';

export default DS.Model.extend({
  username: DS.attr('string'),
  trust: DS.attr('boolean'),
  lastSeen: DS.attr('date'),

  online: computed('lastSeen', {
    get() {
        return moment.duration(moment().diff(get(this, 'lastSeen'))).asMinutes() < 2;
    }
  }),
});
