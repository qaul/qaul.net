import Controller from '@ember/controller';

export default class UserController extends Controller {
  get user() {
    return this.model;
  }
}
