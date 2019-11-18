import { Factory, faker } from 'ember-cli-mirage';

export default Factory.extend({
    id: () => btoa(faker.random.number(Math.pow(2, 128))),
    username: () => faker.random.words(),
    trust: () => faker.random.number({min: 0, max: 2}),
    lastSeen: () => faker.date.recent(),
    identicon: () => faker.image.avatar(),
    avatar: () => faker.image.avatar(),
});
