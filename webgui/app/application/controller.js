import Controller from '@ember/controller';
import { inject as service } from '@ember/service';

export default Controller.extend({
  style: service(),
  intl: service(),
  actions: {
    show(what) {
      this.set('shownav', false);
      this.set('showaside', false);
      this.set('show'+what, true);
    },
    hide() {
      this.set('shownav', false);
      this.set('showaside', false);
    },
  }
});

