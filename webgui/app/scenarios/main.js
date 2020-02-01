import {Scenario} from 'ember-data-factory-guy';

// Just for fun, set the log level ( to 1 ) and see all FactoryGuy response info in console
Scenario.settings({
  logLevel: 1, // 1 is the max for now, default is 0
});

export default class extends Scenario {
  run() {
    console.log('mock');
    this.mockFindAll('message', 3);
  }
}
