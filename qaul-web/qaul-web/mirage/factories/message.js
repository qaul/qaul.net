import { Factory, faker } from 'ember-cli-mirage';

export default Factory.extend({
  senderName: () => faker.name.firstName(),
  content: () => faker.lorem.sentence(),
  timestamp: () => faker.date.recent(),
});
