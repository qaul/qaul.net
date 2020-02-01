import Model, { attr } from '@ember-data/model';

export default Model.extend({
  content: attr('string'),
  senderName: attr('string'),
  senderFingerprint: attr('string'),
  timestamp: attr('date'),
});
