import Controller from '@ember/controller';

export default class MessengerController extends Controller {
  get groups() {
    return this.model;
  }
}
