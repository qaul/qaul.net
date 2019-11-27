import { inject as service } from '@ember/service';
import Component from '@glimmer/component';
import { action } from '@ember/object';

export default class SidepaneLog extends Component {
  @service() intl;

  @action hideLog() {
    document.getElementById("sidepane-log").classList.remove("show");
    document.getElementById("overlay").classList.remove("show");
  }
}
