import EmberRouter from '@ember/routing/router';
import config from './config/environment';

const Router = EmberRouter.extend({
  location: config.locationType,
  rootURL: config.rootURL
});

Router.map(function() {
  this.route('feed');
  this.route('contacts');
  this.route('files');
  this.route('settings');
});

export default Router;
