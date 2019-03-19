import { Factory, faker } from 'ember-cli-mirage';

export default Factory.extend({
  message: () => faker.random.words(),
  suffix: () => faker.system.commonFileExt(),
  size: () => faker.random.number(),
});
