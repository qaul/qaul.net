import RESTAdapter from '@ember-data/adapter/rest';
import { inject as service } from '@ember/service';

export default class ApplicationAdapter extends RESTAdapter {
  namespace = 'rest';

  @service() session;

  get headers() {
    if(this.session.isAuthenticated) {
      return {
        Authorization: JSON.stringify({
          id: this.session.data.authenticated.userId,
          token: this.session.data.authenticated.token,
        }),
      }
    }

    return {};
  }
}
