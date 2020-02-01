import Base from 'ember-simple-auth/authenticators/base';

export default class QaulAuthenticator extends Base {
  restore(data) {
    return data;
  }

  async authenticate(userId, password) {
    const grantResponse = await fetch('/api/grants', {
      method: 'POST',
      headers: { 'Content-Type': 'application/vnd.api+json' },
      body: JSON.stringify({
          data: {
            type: 'grant',
            attributes: {
              secret: password
            },
            relationships: {
              user: {
                data: {
                  type: 'user',
                  id: userId
                }
              }
            }
          }
        })
    });

    if(grantResponse.status !== 201) {
      throw "can not create grant";
    }

    const grantData = await grantResponse.json();

    return {
      token: grantData.data.id,
      userId,
    };
  }

  invalidate(/* data */) {
    return true;
  }
}
