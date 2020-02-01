import FactoryGuy from 'ember-data-factory-guy';
import faker from 'faker';

FactoryGuy.define('user', {
  default: {
    realName: () => faker.random.words(),
    // trust: () => faker.random.number({min: 0, max: 2}),
    // lastSeen: () => faker.date.recent(),
    // identicon: () => faker.image.avatar(),
    // avatar: () => faker.image.avatar(),
  }
});
