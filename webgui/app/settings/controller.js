import Controller from '@ember/controller';
import { inject as service } from '@ember/service';
import { action } from '@ember/object';

export default class SettingsController extends Controller {
  @service() intl;
  @service() style;

  @action
  setLanguage(lang) {
    this.intl.setLocale(lang);
  }

  @action
  setTheme(theme) {
    this.style.setTheme(theme);
  }
}
