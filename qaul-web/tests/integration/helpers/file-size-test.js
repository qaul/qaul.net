
import { moduleForComponent, test } from 'ember-qunit';
import hbs from 'htmlbars-inline-precompile';

moduleForComponent('file-size', 'helper:file-size', {
  integration: true
});

test('it render KiB', function(assert) {
  this.set('inputValue', '1234');
  this.render(hbs`{{file-size inputValue}}`);
  assert.equal(this.$().text().trim(), '1KiB');
});

test('it render KiB', function(assert) {
  this.set('inputValue', '2700877234');
  this.render(hbs`{{file-size inputValue}}`);
  assert.equal(this.$().text().trim(), '2GiB');
});

