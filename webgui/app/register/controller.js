import Controller from '@ember/controller';
import { action } from '@ember/object';
import { inject as service } from '@ember/service';

export default class RegisterController extends Controller {
  @service session;
  password = "123456";

  @action
  async register() {
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

    alert('angemeldet');
  }
}
