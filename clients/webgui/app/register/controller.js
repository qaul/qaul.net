import Controller from '@ember/controller';
import { action } from '@ember/object';
import { inject as service } from '@ember/service';
import { tracked } from '@glimmer/tracking';

export default class RegisterController extends Controller {
  @service session;

  @tracked password = "";
  @tracked realName = "";
  @tracked displayName = "";

  @action
  async register(event) {
    event.preventDefault();

    const createUserResponse = await fetch('/http/users', {
      method: 'POST',
      body: JSON.stringify({
        pw: this.password,
      }),
    });
    if(createUserResponse.status < 200 || createUserResponse.status > 300) {
      console.error("Error while creating a user: " + createUserResponse.status, await createUserResponse.text());
      return;
    }
    const { auth } = await createUserResponse.json();
    await this.session.authenticate('authenticator:qaul', auth.id, this.password);

    const user = await this.store.findRecord('user', auth.id);

    user.realName = this.realName;
    user.displayName = this.displayName,

    await user.save();
  }
}
