'use strict';

const EmberApp = require('ember-cli/lib/broccoli/ember-app');

module.exports = function(defaults) {
  let app = new EmberApp(defaults, {
    'ember-font-awesome': {
      removeUnusedIcons: EmberApp.env() === 'production',
    },
  });

  return app.toTree();
};
