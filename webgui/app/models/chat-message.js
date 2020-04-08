import Model, { attr, belongsTo } from '@ember-data/model';

export default Model.extend({
  content: attr('string'),
  timestamp: attr('date'),

  sender: belongsTo('user'),
  room: belongsTo('chat-room', { inverse: 'messages' }),
});
