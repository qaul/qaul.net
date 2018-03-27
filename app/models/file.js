import DS from 'ember-data';

export default DS.Model.extend({
  message: DS.attr('string'),
  suffix: DS.attr('string'),
  size: DS.attr('number'),
});
