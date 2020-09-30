import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';
import { action } from "@ember/object";

export default class TabletNav extends Component {
  @tracked fakeIncomingOverlay;
  @tracked showAll;
  @tracked showExtendNavButton = true;
  @tracked showShrinkNavButton;
  @tracked overlay;

  @task
  * showNavigation() {
    this.showExtendNavButton = false;
    this.fakeIncomingOverlay = true;
    yield timeout(500);
    this.fakeIncomingOverlay = false;
    this.showAll = true;
    this.showShrinkNavButton = true;
    this.overlay = true;
  }

  @task
  * hideNavigation() {
    this.overlay = false;
    this.showShrinkNavButton = false;
    this.showAll = false;
    this.fakeOutgoingOverlay = true;
    yield timeout(500);
    this.fakeOutgoingOverlay = false;
    this.showExtendNavButton = true;
  }

  @action
  overlayClick() {
    this.hideNavigation.perform();
  }
}
