import Component from '@glimmer/component';
import { task } from 'ember-concurrency-decorators';
import { tracked } from '@glimmer/tracking';

export default class RootComponent extends Component {
  //@tracked showLog;

  @task
  * showLog() {
    //this.showLog = true;
    alert('ok');
  }

  @task
  * hideLog() {
    //this.showLog = false;
  }
}
