import ApplicationAdapter from './application';

export default class UserAdapter extends ApplicationAdapter {
  urlForQuery(query) {
    return {
      contact: () => '/http/contacts',
    }[query.type]();
  }
}
