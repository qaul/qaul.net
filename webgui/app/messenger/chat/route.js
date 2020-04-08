import Route from '@ember/routing/route';
import { hash } from 'rsvp';
export default class ChatRoute extends Route {
  model({ room_id }) {
    return hash({
      room: this.store.findRecord('chat-room', room_id),
      messages: this.store.query('chat-message', { room: room_id }),
    });
  }
}
