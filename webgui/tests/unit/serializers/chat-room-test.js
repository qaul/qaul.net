import { module, test } from 'qunit';
import { setupTest } from 'ember-qunit';

module('Unit | Serializer | chat room', function(hooks) {
  setupTest(hooks);

  // Replace this with your real tests.
  test('it exists', function(assert) {
    let store = this.owner.lookup('service:store');
    let serializer = store.serializerFor('chat-room');

    assert.ok(serializer);
  });

  test('it serializes records', function(assert) {
    let store = this.owner.lookup('service:store');
    let record = store.createRecord('chat-room', {});

    let serializedRecord = record.serialize();

    assert.ok(serializedRecord);
  });
});
