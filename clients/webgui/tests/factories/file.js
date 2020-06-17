import FactoryGuy from 'ember-data-factory-guy';
import faker from 'faker';

FactoryGuy.define('file', {
  default: {
    message: () => faker.random.words(),
    suffix: () => faker.system.commonFileExt(),
    size: () => faker.random.number(),
  }
});
