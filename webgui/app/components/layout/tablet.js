import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';

export default class RootComponent extends Component {
  @tracked fakeIncomingOverlay;
  @tracked showAll;
  @tracked showExtendNavButton = true;
  @tracked showShrinkNavButton;

  @task
  * showNavigation() {
    this.showExtendNavButton = false;
    this.fakeIncomingOverlay = true;
    yield timeout(800);
    this.fakeIncomingOverlay = false;
    this.showAll = true;
    this.showShrinkNavButton = true;
  }

  @task
  * hideNavigation() {
    this.showShrinkNavButton = false;
    this.showAll = false;
    yield timeout(800);
    this.showExtendNavButton = true;
  }
}
