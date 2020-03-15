import EmberRouter from '@ember/routing/router';
import config from './config/environment';

export default class Router extends EmberRouter {
  location = config.locationType;
  rootURL = config.rootURL;
}

Router.map(function() {
  this.route('feed');
  this.route('messenger');
  this.route('users', function() {
    this.route('user', { path: '/:user_id' });
  });
  this.route('files');
  this.route('settings');
  this.route('settings/language');
  this.route('info');
  this.route('register');
});
