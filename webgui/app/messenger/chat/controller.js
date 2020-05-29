import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class MessengerController extends Controller {
  get room() {
    return this.model.room;
  }

  get messages() {
    return this.model.messages;
  }

  @action sendMessage(text) {
    debugger;
    this.store.createRecord('chat-message', {
      room: this.room,
      text,
    }).save();
  }
}
