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

    const secretRequest = await fetch('/api/secrets', {
      method: 'POST',
      headers: { 'Content-Type': 'application/vnd.api+json' },
      body: JSON.stringify({
        data: {
          type: 'secret',
          attributes: { value: this.password }
        }
      })
    });

    if(secretRequest.status !== 201) {
      throw "can not create secret";
    }

    const secretData = await secretRequest.json();
    const userId = secretData.data.relationships.user.data.id;

    await this.session.authenticate('authenticator:qaul', userId, this.password);

    const user = await this.store.findRecord('user', userId);
    user.realName = this.realName;
    user.displayName = this.displayName,

    await user.save();
  }
}
