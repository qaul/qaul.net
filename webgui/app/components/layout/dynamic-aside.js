import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';


export default class MobileComponent extends Component {
  @tracked showAside;
  @tracked animation;

  @task
  * slideIn () {
    this.showAside = true;
    this.animation = 'slide-in';
    yield timeout(500);
    this.animation = '';
  }

  @task
  * slideOut() {
    this.animation = 'slide-out';
    yield timeout(500);
    this.showAside = false;
    this.animation = '';
  }
}
