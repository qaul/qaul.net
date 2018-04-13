import DS from 'ember-data';
import { belongsTo } from 'ember-data/relationships';

export default DS.Model.extend({
  filename: DS.attr('string'),
  extention: DS.attr('string'),
  downloadType: DS.attr('string'),
  downloadStatus: DS.attr('number'),
  size: DS.attr('number'),
  owner: belongsTo('owner'),
});
