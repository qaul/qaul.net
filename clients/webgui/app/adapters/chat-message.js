import ApplicationAdapter from './application';

export default class ChatMessageAdapter extends ApplicationAdapter {
  urlForQuery(query) {
    const path = `/http/chat_message/${query.room}`;
    delete query.room;
    return path;
  }
  urlForCreateRecord(modelName, snapshot) {
    return `/http/chat_message/${snapshot.belongsTo('room').id.replace(/-/g, '')}`
  }
}
