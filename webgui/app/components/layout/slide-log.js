import Component from '@glimmer/component';
import { action } from "@ember/object";


export default class SlideLog extends Component {
  @action
  slideOut() {
    this.args.slideLogOut();
  }
}
