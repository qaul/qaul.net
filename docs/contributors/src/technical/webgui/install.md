# Install EmberJS & Test Web GUI

In order to develop, build and test the qaul.net web GUI 
you need to install EmberJS. This is a step by step guide 
to get you up and running. 


## Prerequisites

You will need the following things properly installed on your computer.

* [Git](https://git-scm.com/)
* [Node.js](https://nodejs.org/)
* [Yarn](https://yarnpkg.com/)
* [Ember CLI](https://ember-cli.com/)
* [Google Chrome](https://google.com/chrome/)

## Installation

The qaul.net web GUI code is in the `webgui` foder of the qaul.net source code repository.

* `git clone git@github.com:qaul/qaul.net.git`
* `cd qaul.net/webgui`
* `yarn`


## Running / Development

* `ember serve`
* Visit your app at [http://localhost:4200](http://localhost:4200).
* Visit your tests at [http://localhost:4200/tests](http://localhost:4200/tests).


### Code Generators

Make use of the many generators for code, try `ember help generate` for more details


### Running Tests

* `ember test`
* `ember test --server`


### Linting

* `npm run lint:hbs`
* `npm run lint:js`
* `npm run lint:js -- --fix`


### Building

* `ember build` (development)
* `ember build --environment production` (production)


### Deploying

* `ember deploy production`


## Further Reading / Useful Links

* [ember.js](https://emberjs.com/)
* [ember-cli](https://ember-cli.com/)
* Development Browser Extensions
  * [ember inspector for chrome](https://chrome.google.com/webstore/detail/ember-inspector/bmdblncegkenkacieihfhpjfppoconhi)
  * [ember inspector for firefox](https://addons.mozilla.org/en-US/firefox/addon/ember-inspector/)
