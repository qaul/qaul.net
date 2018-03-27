import { Factory, faker } from 'ember-cli-mirage';

export default Factory.extend({
    username: () => faker.random.words(),
    trust: () => faker.random.boolean(),
    lastSeen: () => faker.date.recent(),
});
