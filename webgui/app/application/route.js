import Route from '@ember/routing/route';
import { inject as service } from '@ember/service';

export default Route.extend({
  style: service(),
  intl: service(),
  beforeModel() {
    this.intl.setLocale('en');
    this.style.setStyle('light');
  },
})
