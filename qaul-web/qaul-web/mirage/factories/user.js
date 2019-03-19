import { Factory, faker } from 'ember-cli-mirage';

export default Factory.extend({
    id: () => btoa(faker.random.number(Math.pow(2, 128))),
    username: () => faker.random.words(),
    trust: () => faker.random.boolean(),
    lastSeen: () => faker.date.recent(),
});
