const fetch = require('node-fetch');

const API = 'http://127.0.0.1:9900/api';

async function callApi(method, kind, data, auth = undefined) {
  const req = await fetch(API, {
    method: 'POST',
    // headers: {
    //   'Content-Type': 'application/json',
    //   Accept: 'application/json',
    // },
    // body: JSON.stringify({
    //   id: '1', // just always 1 should do it
    //   method,
    //   kind,
    //   data,
    //   auth,
    // }),
  });

  if(req.status < 200 || req.status >= 300) {
    console.error('got not OK ' + req.status);
    throw new Error('can not fetch the API' + await req.text());
  }
  const response = await req.json();

  if(!response.data) {
    throw "no data on the response";
  }

  return response.data;
}

async function test() {
  console.log('Start the tests');

  console.log('TEST: create user');
  const createUserResponse = await callApi('users', 'create', { pw: "1234" });
  const userId = createUserResponse.auth.id;
  
  console.log('TEST: login')
  const loginResponse = await callApi('users', 'login', { user: userId, pw: "1234" });
  const auth = loginResponse.auth;

  console.log('TEST: get own user');
  await callApi('users', 'get', { user: userId }, auth);

  console.log('--------------------------------------------------------------------');
  console.log('From here on I dont really know how the API is an I am just guessing');
  console.log('--------------------------------------------------------------------');

  console.log('TEST: update user')
  await callApi('users', 'update', { id: user, display_name: 'Lux', real_name: 'Lux' }, auth);

  console.log('TEST: get the updated user data')
  const updatedUserData = await callApi('users', 'get', { user: userId }, auth);
  if(updatedUserData.id !== userId) { throw 'got wrong userId back'; }
  if(updatedUserData.display_name !== 'Lux') { throw 'got wrong display_name back'; }
  if(updatedUserData.real_name !== 'Lux') { throw 'got wrong real_name back'; }

  console.log('TEST: create a second user to talk to');
  const createSecondUserResponse = await callApi('users', 'create', { pw: "abcde" });
  const secondUserId = createSecondUserResponse.auth.id;

  console.log('TEST: get the users, we want to display them to the user to start a chat');
  const listUsersResponse = await callApi('users', 'list', { }, auth);
  if(!Array.isArray(listUsersResponse)) { throw 'didnt get an array back' }
  if(listUsersResponse.length !== 2) { throw 'didnt get get exactly 2 users back' }
  
  console.log('TEST: create a chat. I dont pass users yet, because then how would I remove them? Or should I?');
  const createChatResponse = await callApi('chats', 'create', {  }, auth);
  const chatId = createChatResponse.id;

  console.log('TEST: get messages of the chat. There probably are none, and after all noone is in the chat (or just me?).');
  const getMessagesFromChat1 = await callApi('messages', 'query', { chat: chatId }, auth);
  if(!Array.isArray(getMessagesFromChat1)) { throw 'didnt get an array back' }

  console.log('TEST: add someone to the chat. I would assume we need a dedicated resource for this so we can add and later *remove* it?');
  await callApi('userInChat', 'create', { chat: chatId, user: secondUserId }, auth);

  console.log('TEST: create a message');
  await callApi('message', 'create', { chat: chatId, sender: secondUserId }, auth);

  console.log('ok, if this is ever printed then WOAH!!!!!!')
}

test().then(null, err => {
  console.log('');
  console.log('sadly there was an async error:');
  console.log(err);
  process.exit();
})