import DS from 'ember-data';

export default DS.Model.extend({
  content: DS.attr('string'),
  senderName: DS.attr('string'),
  senderFingerprint: DS.attr('string'),
  timestamp: DS.attr('date'),
});
