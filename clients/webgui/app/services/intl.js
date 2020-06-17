import Intl from 'ember-intl/services/intl';
import { inject as service } from '@ember/service';

export default Intl.extend({
  moment: service(),
  setLocale(locale) {
    const dir = locale === 'ar' ? 'rtl' : 'ltr';
    document.documentElement.setAttribute('dir', dir);
    this.set('dir', dir);
    this.set('rtl', dir === 'rtl');
    this.set('ltr', dir === 'ltr');
    this.moment.setLocale(locale);
    this._super(...arguments);
  },
})
