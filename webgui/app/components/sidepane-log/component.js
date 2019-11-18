import Component from '@ember/component';

export default Component.extend({
  actions: {
    hideLog() {
      document.getElementById("sidepane-log").classList.remove("show");
      document.getElementById("overlay").classList.remove("show");
    }
  }
});
