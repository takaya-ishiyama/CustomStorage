/* eslint-disable */
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type CreateUser = {
  __typename?: 'CreateUser';
  id: Scalars['String']['output'];
  session: SessionSchema;
  username: Scalars['String']['output'];
};

export type DirectorySchema = {
  __typename?: 'DirectorySchema';
  id: Scalars['String']['output'];
  name: Scalars['String']['output'];
  parentId?: Maybe<Scalars['String']['output']>;
  userId: Scalars['String']['output'];
};

export type GetNewToken = {
  __typename?: 'GetNewToken';
  accessToken?: Maybe<Scalars['String']['output']>;
  id?: Maybe<Scalars['Int']['output']>;
  refreshToken?: Maybe<Scalars['String']['output']>;
  userId: Scalars['String']['output'];
};

export type GetUser = {
  __typename?: 'GetUser';
  id: Scalars['String']['output'];
  username: Scalars['String']['output'];
};

export type ItemSchema = {
  __typename?: 'ItemSchema';
  directoriesId: Scalars['String']['output'];
  id: Scalars['String']['output'];
  texts: Scalars['String']['output'];
};

export type Login = {
  __typename?: 'Login';
  id: Scalars['String']['output'];
  session: SessionSchema;
  username: Scalars['String']['output'];
};

export type Mutation = {
  __typename?: 'Mutation';
  createDirectory: DirectorySchema;
  createUser: CreateUser;
};


export type MutationCreateDirectoryArgs = {
  name: Scalars['String']['input'];
  parentId?: InputMaybe<Scalars['String']['input']>;
  userId: Scalars['String']['input'];
};


export type MutationCreateUserArgs = {
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export type Query = {
  __typename?: 'Query';
  currentToken?: Maybe<Scalars['String']['output']>;
  getNewToken: GetNewToken;
  getRootDirectory: ServiceSchema;
  getUser: GetUser;
  login: Login;
  loginWithToken: GetUser;
};


export type QueryGetRootDirectoryArgs = {
  userId: Scalars['String']['input'];
};


export type QueryGetUserArgs = {
  id: Scalars['String']['input'];
};


export type QueryLoginArgs = {
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export type ServiceSchema = {
  __typename?: 'ServiceSchema';
  directories?: Maybe<Array<DirectorySchema>>;
  items?: Maybe<Array<ItemSchema>>;
};

export type SessionSchema = {
  __typename?: 'SessionSchema';
  accessToken?: Maybe<Scalars['String']['output']>;
  id?: Maybe<Scalars['Int']['output']>;
  refreshToken?: Maybe<Scalars['String']['output']>;
  userId: Scalars['String']['output'];
};
