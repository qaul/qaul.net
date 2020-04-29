import JSONSerializer from '@ember-data/serializer/json';
import { underscore } from '@ember/string';

export default class ApplicationSerializer extends JSONSerializer {
  keyForAttribute(attr) {
    return underscore(attr);
  }
  normalizeSingleResponse (store, primaryModelClass, payload, id, requestType) {
    // we convert from { user: [ { ...data... } ] } to { ...data... }
    return super.normalizeSingleResponse(store, primaryModelClass, payload[primaryModelClass.modelName][0], id, requestType);
  }
}
