import ApplicationSerializer from './application';
import { underscore } from '@ember/string';

export default class ChatMessageSerializer extends ApplicationSerializer {

  normalizeSingleResponse (store, primaryModelClass, payload, id, requestType) {
    payload[underscore(primaryModelClass.modelName)] = payload[underscore(primaryModelClass.modelName)][0];

    payload[underscore(primaryModelClass.modelName)].room = payload[underscore(primaryModelClass.modelName)].room.Id;

    return super.normalizeSingleResponse(store, primaryModelClass, payload, id, requestType);
  }
}
