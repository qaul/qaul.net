import Controller from '@ember/controller';
import { action } from '@ember/object';
import { tracked } from '@glimmer/tracking';
import { inject as service } from '@ember/service';

export default class LoginController extends Controller {
  @service session;

  @tracked user = null;
  @tracked password = "";

  @action selectUser(event) {
    this.user = this.model.find(user => user.id === event.target.value);
  }

  @action async submit(event) {
    event.preventDefault();

    await this.session.authenticate('authenticator:qaul', this.user.id, this.password);
  }
}
