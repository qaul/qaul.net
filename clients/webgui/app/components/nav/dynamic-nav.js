import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';
import { action } from "@ember/object";

export default class DynamicNav extends Component {
  @tracked showNav;
  @tracked animation;
  @tracked overlay;

  @task
  * slideIn() {
    this.showNav = true;
    yield timeout(10);
    this.animation = 'slide-in';
    this.overlay = true;
  }

  @task
  * slideOut() {
    this.animation = '';
    this.overlay = false;
    yield timeout(300);
    this.showNav = false;
  }

  @action
  overlayClick() {
    this.slideOut.perform();
  }
}
