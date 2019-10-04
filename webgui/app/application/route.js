import Route from '@ember/routing/route';
import { inject as service } from '@ember/service';

export default Route.extend({
  style: service(),
  beforeModel() {
    this.style.setStyle('light');
  },
})
