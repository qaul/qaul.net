import Route from '@ember/routing/route';

export default class UserRoute extends Route {
  async model({ user_id }) {
    const users = await this.store.query('user', { type: 'contact' });
    return users.find(u => u.id === user_id);
  }
}
