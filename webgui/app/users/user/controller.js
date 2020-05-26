import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class UserController extends Controller {
  get user() {
    return this.model;
  }

  @action startChart() {
    this.store.createRecord('chat-room', {
      name: 'foo',
      users: [this.user],
    }).save();
  }
}
