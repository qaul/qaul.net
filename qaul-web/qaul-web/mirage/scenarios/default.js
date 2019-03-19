import {faker} from 'ember-cli-mirage';

export default function(server) {
  faker.seed(1337);
  /*
    Seed your development database using your factories.
    This data will not be loaded in your tests.
  */

  // server.createList('post', 10);
  server.createList('message', 10);
  server.createList('file', 10);
  server.createList('user', 1, {
    lastSeen: new Date(),
  });
  server.createList('user', 10);
}
