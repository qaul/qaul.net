import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class UserController extends Controller {
  get user() {
    return this.model;
  }

  @action async startChart() {
    const room = this.store.createRecord('chat_room', {
      name: 'foo',
      users: [this.user],
    })
    await room.save();

    this.transitionToRoute('messenger.chat', room.id);
  }
}
