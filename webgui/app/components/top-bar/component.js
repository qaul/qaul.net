import Component from '@ember/component';

export default Component.extend({
  actions: {
    showMenu() {
      document.getElementById("sidepane-log").classList.remove("show");
      document.getElementById("sidepane-menu").className += " show";
      document.getElementById("overlay").className += " show";
    },
    showLog() {
      document.getElementById("sidepane-menu").classList.remove("show");
      document.getElementById("sidepane-log").className += " show";
      document.getElementById("overlay").className += " show";
    },
    hidePanes() {
      document.getElementById("sidepane-menu").classList.remove("show");
      document.getElementById("sidepane-log").classList.remove("show");
      document.getElementById("overlay").classList.remove("show");
    }
  }
});
