import FactoryGuy from 'ember-data-factory-guy';
import faker from 'faker';

FactoryGuy.define('chat_message', {
  default: {
    content: () => faker.lorem.sentence(),
    timestamp: () => faker.date.recent(),

    sender: () => FactoryGuy.belongsTo('user'),
  }
});
