import Controller from '@ember/controller';
import { inject as service } from '@ember/service';

export default Controller.extend({
  intl: service(),
  style: service(),
  actions: {
    setLanguage(lang) {
      this.intl.setLocale(lang);
    },
    setTheme(theme) {
      this.style.setTheme(theme);
    },
  }
});
