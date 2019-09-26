import Controller from '@ember/controller';
import { computed } from '@ember/object'

export default Controller.extend({
    messages: computed('model.@each.timestamp', {
        get() {
            return this.model.sortBy('timestamp').reverse();
        }
    }),
    actions: {
        submit(/*content*/) {
            // const store = get(this, 'store');
            // const newMsg = store.createRecord('message', {
            //     senderName: 'Lux',
            //     timestamp: new Date(),
            //     content
            // });
        }
    }
});
