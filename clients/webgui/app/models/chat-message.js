import Model, { attr, belongsTo } from '@ember-data/model';

export default class ChatMessage extends Model {
  @attr text;
  @attr content;
  @attr('date') timestamp;
  @belongsTo('chat_room', { inverse: 'messages' }) room;
}

// content: attr('string'),
// timestamp: attr('date'),

// sender: belongsTo('user'),
