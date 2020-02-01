import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';

export default class FeedController extends Controller {
  @tracked newMsg;

  get messages() {
    return this.model.sortBy('timestamp').reverse();
  }

  @action
  enterMessage(event) {
    this.newMsg = event.target.value;
  }

  @action
  submit() {}
}
