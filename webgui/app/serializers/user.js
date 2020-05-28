import ApplicationSerializer from './application';

export default class UserSerializer extends ApplicationSerializer {
  normalizeSingleResponse (store, primaryModelClass, payload, id, requestType) {
    // we convert from { user: [ { ...data... } ] } to { ...data... }
    return super.normalizeSingleResponse(store, primaryModelClass, payload[primaryModelClass.modelName][0], id, requestType);
  }
}
