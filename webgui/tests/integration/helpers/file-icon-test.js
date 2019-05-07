
import { moduleForComponent, test } from 'ember-qunit';
import hbs from 'htmlbars-inline-precompile';

moduleForComponent('file-icon', 'helper:file-icon', {
  integration: true
});

// Replace this with your real tests.
test('it renders', function(assert) {
  this.render(hbs`{{file-icon 'jpeg'}}`);

  assert.equal(this.$().text().trim(), 'image');
});

