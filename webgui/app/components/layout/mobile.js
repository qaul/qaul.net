import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';
import { action } from "@ember/object";


export default class MobileComponent extends Component {
  @tracked showNavOverlay;
  @tracked showNav;
  @tracked showLogOverlay;
  @tracked showLog;
  @tracked showShadowOverlay;

  @task
  * slideNavIn() {
    yield timeout(1); // Todo: add animation
    this.showNavOverlay = true;
    this.showNav = true;
    this.showShadowOverlay = true;
  }

  @task
  * slideNavOutTask() {
    this.showNav = false;
    this.showShadowOverlay = false;
    yield timeout(600);
    this.showNavOverlay = false;
  }

  @action
  slideNavOut() {
    this.showNav = false;
    this.showNavOverlay = false;
    this.showShadowOverlay = false;
    //this.slideNavOutTask();
  }

  @task
  * slideLogIn() {
    yield timeout(1); // Todo: add animation
    this.showLogOverlay = true;
    this.showLog = true;
    this.showShadowOverlay = true;
  }

  @task
  * slideLogOutTask() {
    this.showLog = false;
    this.showShadowOverlay = false;
    yield timeout(600);
    this.showLogOverlay = false;
  }

  @action
  slideLogOut() {
    this.showLog = false;
    this.showLogOverlay = false;
    this.showShadowOverlay = false;
    //this.slideLogOutTask();
  }

  @task
  * slideOut() {
    yield timeout(1); // Todo: add animation
    if (this.showNav) this.slideNavOut();
    if (this.showLog) this.slideLogOut();
  }
}
