import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class UserController extends Controller {
  get user() {
    return this.model;
  }

  @action async startChart() {
    const room = this.store.createRecord('chat-room', {
      name: 'foo',
      users: [this.user],
    })
    await room.save();

    this.store.createRecord('chat-message', {
      room,
      text: 'hallo du',
    }).save();
  }
}
