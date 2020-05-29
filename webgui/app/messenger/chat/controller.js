import Controller from '@ember/controller';
import { action } from '@ember/object';
import { later } from '@ember/runloop';

export default class MessengerController extends Controller {
  get room() {
    return this.model.room;
  }

  get messages() {
    return this.model.messages.sortBy('timestamp');
  }

  @action async sendMessage(text) {
    await this.store.createRecord('chat-message', {
      room: this.room,
      text,
    }).save();
    this.send('runRefresh');
  }
}
