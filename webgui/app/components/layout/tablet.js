import Component from '@glimmer/component';
import { timeout } from 'ember-concurrency';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';
import { action } from "@ember/object";


export default class Tablet extends Component {
  @tracked showLogOverlay;
  @tracked showLog;
  @tracked showShadowOverlay;

  @action
  slideLogIn() {
    this.showLogOverlay = true;
    this.showLog = true;
    this.showShadowOverlay = true;
  }

  @action
  slideLogOut() {
    this.showLog = false;
    this.showLogOverlay = false;
    this.showShadowOverlay = false;
  }
}
