import FactoryGuy from 'ember-data-factory-guy';
import faker from 'faker';

FactoryGuy.define('message', {
  default: {
    senderName: () => faker.name.firstName(),
    content: () => faker.lorem.sentence(),
    timestamp: () => faker.date.recent(),
  }
});
