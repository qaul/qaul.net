import Component from '@glimmer/component';
import { action } from "@ember/object";


export default class SlideNav extends Component {

  @action
  slideOut() {
    this.args.slideNavOut();
  }
}
