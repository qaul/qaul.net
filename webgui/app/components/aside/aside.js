import { inject as service } from '@ember/service';
import Component from '@glimmer/component';
import { action } from '@ember/object';

export default class Aside extends Component {
  @service() intl;
  @service() session;

  @action
  logout() {
    this.session.invalidate();
  }
}
