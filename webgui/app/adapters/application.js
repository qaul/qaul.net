import RESTAdapter from '@ember-data/adapter/rest';
import { inject as service } from '@ember/service';

function serializeIntoHash(store, modelClass, snapshot, options = { includeId: true }) {
  const serializer = store.serializerFor(modelClass.modelName);

  if (typeof serializer.serializeIntoHash === 'function') {
    const data = {};
    serializer.serializeIntoHash(data, modelClass, snapshot, options);
    return data;
  }

  return serializer.serialize(snapshot, options);
}

export default class ApplicationAdapter extends RESTAdapter {
  namespace = 'rest';

  @service() session;

  get headers() {
    if(this.session.isAuthenticated) {
      return {
        Authorization: JSON.stringify({
          id: this.session.data.authenticated.userId,
          token: this.session.data.authenticated.token,
        }),
      }
    }

    return {};
  }

  updateRecord(store, type, snapshot) {
    const data = serializeIntoHash(store, type, snapshot, {});

    let id = snapshot.id;
    let url = this.buildURL(type.modelName, id, snapshot, 'updateRecord');

    return this.ajax(url, 'PATCH', { data });
  }
}
