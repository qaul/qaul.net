import Route from '@ember/routing/route';
import { hash } from 'rsvp';
export default class ChatRoute extends Route {
  model({ group_id }) {
    return hash({
      group: this.store.findRecord('group', group_id),
      messages: this.store.query('chatmessage', { group: group_id }),
    });
  }
}
