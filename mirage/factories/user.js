import { Factory } from 'ember-cli-mirage';

export default Factory.extend({
    username: () => faker.random.words(),
    trust: () => faker.system.commonFileExt(),
    online: () => faker.random.number(),
    lastSeen: () => faker.date.recent(),
});
