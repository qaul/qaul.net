import Component from '@glimmer/component';
import { action } from "@ember/object";

export default class Overlay extends Component {
  @action
  overlayClick() {
    this.args.overlayClick();
  }
}
