import Component from '@glimmer/component';
import { inject as service } from '@ember/service';
import { action } from '@ember/object';

export default class ItemRadio extends Component {
  @service() intl;

  @action
  setLanguage(lang) {
    this.intl.setLocale(lang);
  }

  get languageSelected() {
    return this.intl.get('primaryLocale');
  }
}
