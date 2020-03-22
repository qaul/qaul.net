import Service from '@ember/service';

const light = {};
/* over-all background */
light['q-bg'] = '#ffffff';
light['q-bg-hover'] = '#f6f6f6';

/* theme color */
light['q-theme-strong'] = '#0089c7';
light['q-theme-strong-hover'] = '#0076ab';
light['q-theme-middle'] = '#d1e4f1';
light['q-theme-light'] = '#d1e4f1';

/* text color */
light['q-color'] = '#666';
light['q-color-title'] = '#333';
light['q-color-inverse'] = '#fff';

/* hyperlinks */
light['q-a'] = '#6399cc';
light['q-a-hover'] = '#2d6dad'; /* hover when there is underlining */
light['q-a-hover-nounderline'] = '#000'; /* hover when there is no underlining */

/* user availability */
light['q-available'] = 'limegreen';


light['q-border'] = '#989898';
light['q-border-light'] = '#ddd';

light['q-nav-bg'] = '#f6f6f6';
light['q-nav-bg-hover'] = '#ececec';
light['q-nav-border'] = '#cccccc';
light['q-nav-color'] = '#363636';
light['q-nav-bg-active'] = light['q-theme-strong'];
light['q-nav-bg-active-hover'] = light['q-theme-strong'];
light['q-nav-icon'] = '#8a8a8a';
light['q-nav-icon-shadow'] = '#1c1c1c';
light['q-input-hint'] = '#cccccc'; /* hint text in the unselected input text field */
light['q-bar-bg'] = '#666';
light['q-bar-border'] = '#aaa';

const dark = {};
dark['q-bg'] = '#222';
dark['q-bg-hover'] = '#2f2f2f';
dark['q-bg-middle'] = '#333';
dark['q-bg-middle-hover'] = '#3f3f3f';
dark['q-bg-strong'] = '#444';
dark['q-bg-strong-hover'] = '#4f4f4f';

  /* theme color */
dark['q-theme-strong'] = '#0076ab';
dark['q-theme-strong-hover'] = '#0089c7';
dark['q-theme-middle'] = '#386377';
dark['q-theme-light'] = '#4b5962';

  /* borders */
dark['q-border'] = '#666';
dark['q-border-light'] = '#444';

  /* text color */
dark['q-color'] = '#aaa';
dark['q-color-title'] = '#ddd';

  /* hyperlinks */
dark['q-a'] = '#6399cc';
dark['q-a-hover'] = '#73b1ef'; /* hover when there is underlining */
dark['q-a-hover-nounderline'] = '#eee'; /* hover when there is no underlining */

  /* user availability */
dark['q-available:limegreen'] = '';

dark['q-nav-bg'] = dark['q-bg-middle'];
dark['q-nav-bg-hover'] = dark['q-bg-middle-hover'];
dark['q-nav-border'] = dark['q-border-light'];
dark['q-nav-color'] = '#ddd';
dark['q-nav-bg-active'] = dark['q-theme-strong'];
dark['q-nav-bg-active-hover'] = dark['q-theme-strong-hover'];
dark['q-nav-icon'] = '#8a8a8a';
dark['q-nav-icon-shadow'] = '#1c1c1c';
dark['q-input-hint'] = '#666'; /* hint text in the unselected input text field */
dark['q-bar-bg'] = '#666';
dark['q-bar-border'] = '#aaa';

const themes = { dark, light };

export default Service.extend({
  activeStyle: Object.freeze({}),
  setTheme(theme) {
    this.set('activeStyle', themes[theme]);
  },
});
