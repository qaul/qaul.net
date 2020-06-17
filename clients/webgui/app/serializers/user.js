import ApplicationSerializer from './application';
import { underscore } from '@ember/string';

export default class UserSerializer extends ApplicationSerializer {
  normalizeSingleResponse (store, primaryModelClass, payload, id, requestType) {
    payload[underscore(primaryModelClass.modelName)] = payload[underscore(primaryModelClass.modelName)][0];

    return super.normalizeSingleResponse(store, primaryModelClass, payload, id, requestType);
  }
}
