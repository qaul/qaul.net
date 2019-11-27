import { inject as service } from '@ember/service';
import Component from '@glimmer/component';
// import { action } from '@ember/object';

export default class SidepaneLog extends Component {
  @service() intl;
}
