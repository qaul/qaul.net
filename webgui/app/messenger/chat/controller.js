import Controller from '@ember/controller';

export default class MessengerController extends Controller {
  get group() {
    return this.model;
  }
}
