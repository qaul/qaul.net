import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { action } from "@ember/object";


export default class SlideNav extends Component {

  @action
  slideOut() {
    this.args.slideNavOut();
  }
}
