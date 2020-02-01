import JSONAPIAdapter from '@ember-data/adapter/json-api';
import { inject as service } from '@ember/service';

export default class ApplicationAdapter extends JSONAPIAdapter {
  namespace = 'api';

  @service() session;

  get headers() {
    if(this.session.isAuthenticated) {
      return {
        Authorization: `Bearer ${this.session.data.authenticated.token}`,
      }
    }

    return {};
  }
}
