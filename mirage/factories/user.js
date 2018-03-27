import { Factory, faker } from 'ember-cli-mirage';
import { computed, get } from '@ember/object';
import moment from 'moment';

export default Factory.extend({
    username: () => faker.random.words(),
    trust: () => faker.random.boolean(),
    lastSeen: () => faker.date.recent(),

    online: computed('lastSeen', {
        get() {
            return moment().diff(get(this, 'lastSeen')).asMinutes() < 2;
        }
    }),
});
