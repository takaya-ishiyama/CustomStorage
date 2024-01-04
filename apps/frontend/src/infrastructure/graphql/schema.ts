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

export const query = {
	login_with_token: () => `
		query {
			loginWithToken {
				id
				username
			}
		`,
	get_new_token: () => `
	query{
		getNewToken {
			accessToken
			refreshToken
		}
	}
	`,
};
