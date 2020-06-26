import { module, test } from 'qunit';
import { setupRenderingTest } from 'ember-qunit';
import { render } from '@ember/test-helpers';
import hbs from 'htmlbars-inline-precompile';

module('helper:file-size', function(hooks) {
  setupRenderingTest(hooks);

  test('it render KiB', async function(assert) {
    this.set('inputValue', '1234');
    await render(hbs`<div id="size">{{file-size inputValue}}</div>`);
    assert.dom('#size').hasText('1KiB');
  });

  test('it render KiB', async function(assert) {
    this.set('inputValue', '2700877234');
    await render(hbs`<div id="size">{{file-size inputValue}}</div>`);
    assert.dom('#size').hasText('2GiB');
  });
});

