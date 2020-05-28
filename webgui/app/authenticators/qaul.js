import Base from 'ember-simple-auth/authenticators/base';

export default class QaulAuthenticator extends Base {
  async restore(data) {
    const validateTokenResponse = await fetch('/rest/validate_token', {
      headers: {
        Authorization: JSON.stringify(data),
      }
    });
    if(validateTokenResponse.status !== 200) {
      throw "login not valid"
    }

    return data;
  }

  async authenticate(id, pw) {
    const loginResponse = await fetch('/rest/login', {
      method: 'POST',
      body: JSON.stringify({ id, pw }),
    });
    if(loginResponse.status < 200 || loginResponse.status >= 300) {
      throw new Error("error during login" + await loginResponse.text());
    }

    return (await loginResponse.json()).auth;
  }

  async invalidate({ token }) {
    await fetch(`/api/grants/${token}`, {
      method: 'DELETE',
      headers: {
        'Content-Type': 'application/vnd.api+json',
        Authorization: `Bearer ${token}`
      },
    });
  }
}
