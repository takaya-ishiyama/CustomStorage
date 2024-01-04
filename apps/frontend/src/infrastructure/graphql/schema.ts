export const mutation = {};

export const query = {
	login: () => `
	query LoginQuery($username: String!, $password: String!) {
	  login(username: $username, password: $password) {
		id
		username
		accessToken
		refreshToken
	  }
	}
  `,
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
