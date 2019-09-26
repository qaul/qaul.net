import Service from '@ember/service';
import { set } from "@ember/object";

export default Service.extend({
  init() {
    this._super(...arguments);
    const update = () => {
      if(window.innerWidth != this.width) {
        set(this, 'width', window.innerWidth);
      }

      if(window.innerHeight != this.height) {
        set(this, 'height', window.innerHeight);
      }

      window.requestAnimationFrame(update);
    }

    update();
  }
});
