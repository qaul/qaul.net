import Component from '@glimmer/component';
import { inject as service } from '@ember/service';
import { action } from '@ember/object';

export default class ItemTheme extends Component {
  @service() style;

  @action
  setTheme(theme) {
    this.style.setTheme(theme);
  }
}
