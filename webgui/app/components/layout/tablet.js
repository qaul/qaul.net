import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';

export default class RootComponent extends Component {
  @tracked fakeIncomingOverlay;
  @tracked showAll;

  @task
  * showNavigation() {
    this.fakeIncomingOverlay = true;
    yield timeout(800);
    this.fakeIncomingOverlay = false;
    this.showAll = true;
  }
}
