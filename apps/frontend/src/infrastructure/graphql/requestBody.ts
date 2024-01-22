export const mutation = {
	register: () => `
	mutation RegisterMutation($username: String!, $password: String!) {
		createUser(username: $username, password: $password) {
			id
			username
			session {
				accessToken
				refreshToken
			}
		}
	}
	`,
	create_directory: () => `
	mutation CreateDirectoryMutation($userId: String!, $name: String!, $parentId: String) {
		createDirectory(userId: $userId, name: $name, parentId: $parentId){
		  id
		  userId
		  name
		  parentId
		}
	  }
	`,
};

export const query = {
	login: () => `
	query LoginQuery($username: String!, $password: String!) {
	  login(username: $username, password: $password) {
		id
		username
		session {
			accessToken
			refreshToken
		}
	  }
	}
  `,
	login_with_token: () => `
		query {
			loginWithToken {
				id
				username
			}
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

	get_root_directory: () => `
		query GetFRootDirectory($userId: String!) {
			getRootDirectory(userId: $userId){
				directories{
					id
					userId
					parentId
					name
				}
				items{
					id
					directoriesId
					texts
				}
			}
		}
	`,
};
