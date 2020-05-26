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
  normalizeArrayResponse(store, primaryModelClass, payload, id, requestType) {
    return super.normalizeArrayResponse(store, primaryModelClass, payload[primaryModelClass.modelName], id, requestType);
  }
  serializeAttribute(snapshot, json, key) {
    super.serializeAttribute(...arguments);
    json[this.keyForAttribute(key)] = { set: json[this.keyForAttribute(key)] };
  }

  serialize(snapshot, options) {
    let json = {};

    if (options && options.includeId) {
      const id = snapshot.id;
      if (id) {
        json[this.primaryKey] = id;
      }
    }

    const changedAttributes = Object.keys(snapshot.changedAttributes());
    snapshot.eachAttribute((key, attribute) => {
      if(changedAttributes.includes(key)) {
        this.serializeAttribute(snapshot, json, key, attribute);
      }
    });

    snapshot.eachRelationship((key, relationship) => {
      if (relationship.kind === 'belongsTo') {
        this.serializeBelongsTo(snapshot, json, relationship);
      } else if (relationship.kind === 'hasMany') {
        this.serializeHasMany(snapshot, json, relationship);
      }
    });

    console.log(json);
    return json;
  }
}
