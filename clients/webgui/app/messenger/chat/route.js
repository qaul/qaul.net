import Route from '@ember/routing/route';
import { hash } from 'rsvp';
import { action } from '@ember/object';
export default class ChatRoute extends Route {
  model({ room_id }) {
    return hash({
      room: this.store.findRecord('chat-room', room_id),
      messages: this.store.query('chat-message', { 'chat-room': room_id }),
    });
  }

  activate() {
    this.refreshInterval = setInterval(() => this.refresh(), 500);
  }

  deactivate() {
    clearInterval(this.refreshInterval);
  }

  @action runRefresh() {
    this.refresh();
  }
}
