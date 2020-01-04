import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';


export default class RootComponent extends Component {
  @tracked showNavOverlay;
  @tracked showNav;
  @tracked showLogOverlay;
  @tracked showLog;

  @task
  * slideNavIn() {
    this.showNavOverlay = true;
    this.showNav = true;
  }

  @task
  * slideNavOut() {
    this.showNav = false;
    yield timeout(600);
    this.showNavOverlay = false;
  }

  @task
  * slideLogIn() {
    this.showLogOverlay = true;
    this.showLog = true;
  }

  @task
  * slideLogOut() {
    this.showLog = false;
    yield timeout(600);
    this.showLogOverlay = false;
  }
}
