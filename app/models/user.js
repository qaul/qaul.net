import DS from 'ember-data';

export default DS.Model.extend({
  username: DS.attr('string'),
  trust: DS.attr('boolean'),
  lastSeen: Ds.attr('date'),
});
