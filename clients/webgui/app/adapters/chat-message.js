import ApplicationAdapter from './application';

export default class ChatMessageAdapter extends ApplicationAdapter {
  urlForQuery(query) {
    const path = `/http/chat-messages/${query.room}`;
    delete query.room;
    return path;
  }
  urlForCreateRecord(modelName, snapshot) {
    return `/http/chat-messages/${snapshot.belongsTo('room').id.replace(/-/g, '')}`
  }
}
