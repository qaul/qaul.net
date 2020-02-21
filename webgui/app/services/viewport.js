import Service from '@ember/service';
import { tracked } from '@glimmer/tracking';

export default class ViewportService extends Service {
  @tracked width;
  @tracked height;

  constructor() {
    super(...arguments);

    const update = () => {
      if(window.innerWidth != this.width) {
        this.width = window.innerWidth;
      }

      if(window.innerHeight != this.height) {
        this.height = window.innerHeight;
      }

      window.requestAnimationFrame(update);
    }

    update();
  }

  get layout() {
    if(this.width < 768) {
      return 'mobile';
    }
    else if(this.width < 950) {
      return 'tablet';
    }
    else if(this.width < 1200) {
      return 'largetablet';
    }

    return 'desktop';
  }
}
