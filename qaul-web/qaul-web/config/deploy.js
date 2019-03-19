/* eslint-env node */
'use strict';

module.exports = function(deployTarget) {
  let ENV = {
    build: {},
    // include other plugin configuration that applies to all deploy targets here
    gzip: {}
  };


  ENV.build.outputPath = process.env.DEPLOY_PATH || 'deploy';
  ENV.gzip.filePattern = '\*\*/\*.{js,css,json,ico,html,svg,otf,eot,ttf,woff,woff2}';
  ENV.gzip.keep = 'true';

  if (deployTarget === 'development') {
    ENV.build.environment = 'development';
    // configure other plugins for development deploy target here
  }

  if (deployTarget === 'staging') {
    ENV.build.environment = 'production';
    // configure other plugins for staging deploy target here
  }

  if (deployTarget === 'production') {
    ENV.build.environment = 'production';
    // configure other plugins for production deploy target here
  }

  // Note: if you need to build some configuration asynchronously, you can return
  // a promise that resolves with the ENV object instead of returning the
  // ENV object synchronously.
  return ENV;
};
