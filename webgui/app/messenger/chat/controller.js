import Controller from '@ember/controller';

export default class MessengerController extends Controller {
  get room() {
    return this.model.room;
  }

  get messages() {
    return this.model.messages;
  }
}
