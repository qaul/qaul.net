import Component from '@glimmer/component';
import { inject as service } from '@ember/service';

export default class RootComponent extends Component {
  @service() viewport;
  @service() session;

  get layoutComponentName() {
    return `layout/${this.viewport.layout}`
  }
}
