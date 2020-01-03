import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';


export default class RootComponent extends Component {
  @tracked showNavOverlay;
  @tracked showLogOverlay;

  @task
  * showNavigation() {
    this.showNavOverlay = true;
    this.showNav = true;
  }

  @task
  * hideNavigation() {
    this.showNav = false;
    yield timeout(600);
    this.showNavOverlay = false;
  }

  @task
  * showLog() {
    this.showLogOverlay = true;
    this.showLog = true;
  }

  @task
  * hideLog() {
    this.showLog = false;
    yield timeout(600);
    this.showLogOverlay = false;
  }
}
