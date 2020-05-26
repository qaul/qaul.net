import ApplicationAdapter from './application';

export default class ChatMessageAdapter extends ApplicationAdapter {
  urlForCreateRecord(modelName, snapshot) {
    debugger;
    return `/rest/chat-messages/${snapshot.belongsTo('room').id.replace(/-/g, '')}`
  }
}
