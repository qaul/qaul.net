import Model, { belongsTo, attr } from '@ember-data/model';

export default Model.extend({
  filename: attr('string'),
  extention: attr('string'),
  downloadType: attr('string'),
  downloadStatus: attr('number'),
  size: attr('number'),
  owner: belongsTo('owner'),
});
