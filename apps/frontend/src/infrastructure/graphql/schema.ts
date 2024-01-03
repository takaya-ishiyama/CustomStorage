export const mutation = {
	login: () => `
	mutation LoginMutation($username: String!, $password: String!) {
	  login(username: $username, password: $password) {
		id
		username
		accessToken
		refreshToken
	  }
	}
  `,
};
