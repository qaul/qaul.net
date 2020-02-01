import JSONAPISerializer from '@ember-data/serializer/json-api';
import { underscore } from '@ember/string';
import { singularize } from 'ember-inflector';

export default class ApplicationSerializer extends JSONAPISerializer {
  keyForAttribute(attr) {
    return underscore(attr);
  }
  payloadKeyFromModelName(modelName) {
    return singularize(modelName);
  }
}
