import Model, { attr } from '@ember-data/model';

export default Model.extend({
  name: attr('string'),
  shared: attr('boolean'),
});
