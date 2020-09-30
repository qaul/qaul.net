import Controller from '@ember/controller';

export default class MessengerController extends Controller {
  get rooms() {
    return this.model;
  }
}
