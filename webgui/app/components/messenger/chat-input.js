import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';

export default class ChatInputComponent extends Component {
  @tracked value = "";
  @action submit() {
    this.args.sendMessage(this.value);
    this.value = "";
  }
}
