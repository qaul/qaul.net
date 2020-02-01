import {Scenario, getPretender} from 'ember-data-factory-guy';
import 'faker'; // used in the factories

// Just for fun, set the log level ( to 1 ) and see all FactoryGuy response info in console
Scenario.settings({
  logLevel: 1, // 1 is the max for now, default is 0
});

export default class extends Scenario {
  run() {
    // Passthrough 'data:' requests.

    // this.mockFindAll('message', 10);
    // this.mockFindAll('user', 10);
    // this.mockFindAll('file', 10);

    // getPretender().get('/api/secrets', getPretender().passthrough);
  }
}
