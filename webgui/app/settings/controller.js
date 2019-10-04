import Controller from '@ember/controller';
import { inject as service } from '@ember/service';

export default Controller.extend({
  intl: service(),
  actions: {
    setLanguage(lang) {
      this.intl.setLocale(lang);
    }
  }
});
