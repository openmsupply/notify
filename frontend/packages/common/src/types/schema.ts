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
  DateTime: { input: string; output: string; }
};

export type AcceptUserInviteInput = {
  displayName: Scalars['String']['input'];
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export type AccessDenied = LogoutErrorInterface & {
  __typename: 'AccessDenied';
  description: Scalars['String']['output'];
  fullError: Scalars['String']['output'];
};

export type AuthToken = {
  __typename: 'AuthToken';
  /** Bearer token */
  token: Scalars['String']['output'];
};

export type AuthTokenError = {
  __typename: 'AuthTokenError';
  error: AuthTokenErrorInterface;
};

export type AuthTokenErrorInterface = {
  description: Scalars['String']['output'];
};

export type AuthTokenResponse = AuthToken | AuthTokenError;

export type CreateRecipientInput = {
  id: Scalars['String']['input'];
  name: Scalars['String']['input'];
  notificationType: NotificationTypeNode;
  toAddress: Scalars['String']['input'];
};

export type CreateRecipientResponse = RecipientNode;

export type CreateUserAccountInput = {
  displayName?: InputMaybe<Scalars['String']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  password: Scalars['String']['input'];
  permissions: Array<PermissionNode>;
  username: Scalars['String']['input'];
};

export type CreateUserAccountResponse = UserAccountNode;

export type DatabaseError = RefreshTokenErrorInterface & {
  __typename: 'DatabaseError';
  description: Scalars['String']['output'];
  fullError: Scalars['String']['output'];
};

export type DeleteRecipientResponse = DeleteResponse;

export type DeleteResponse = {
  __typename: 'DeleteResponse';
  id: Scalars['String']['output'];
};

export type DeleteUserAccountResponse = DeleteResponse;

export type EqualFilterLogTypeInput = {
  equalAny?: InputMaybe<Array<LogNodeType>>;
  equalTo?: InputMaybe<LogNodeType>;
  notEqualTo?: InputMaybe<LogNodeType>;
};

export type EqualFilterNotificationTypeInput = {
  equalAny?: InputMaybe<Array<NotificationTypeNode>>;
  equalTo?: InputMaybe<NotificationTypeNode>;
  notEqualTo?: InputMaybe<NotificationTypeNode>;
};

export type EqualFilterStringInput = {
  equalAny?: InputMaybe<Array<Scalars['String']['input']>>;
  equalTo?: InputMaybe<Scalars['String']['input']>;
  notEqualTo?: InputMaybe<Scalars['String']['input']>;
};

export type FullMutation = {
  __typename: 'FullMutation';
  /** Updates user account based on a token and their information (Response to initiate_user_invite) */
  acceptUserInvite: InviteUserResponse;
  createRecipient: CreateRecipientResponse;
  createUserAccount: CreateUserAccountResponse;
  deleteRecipient: DeleteRecipientResponse;
  deleteUserAccount: DeleteUserAccountResponse;
  /**
   * Initiates the password reset flow for a user based on email address
   * The user will receive an email with a link to reset their password
   */
  initiatePasswordReset: PasswordResetResponse;
  /** Invites a new user to the system */
  initiateUserInvite: InviteUserResponse;
  /** Resets the password for a user based on the password reset token */
  resetPasswordUsingToken: PasswordResetResponse;
  updateRecipient: UpdateRecipientResponse;
  updateUserAccount: UpdateUserAccountResponse;
  /** Validates Password Reset Token */
  validatePasswordResetToken: PasswordResetResponse;
};


export type FullMutationAcceptUserInviteArgs = {
  input: AcceptUserInviteInput;
  token: Scalars['String']['input'];
};


export type FullMutationCreateRecipientArgs = {
  input: CreateRecipientInput;
};


export type FullMutationCreateUserAccountArgs = {
  input: CreateUserAccountInput;
};


export type FullMutationDeleteRecipientArgs = {
  recipientId: Scalars['String']['input'];
};


export type FullMutationDeleteUserAccountArgs = {
  userAccountId: Scalars['String']['input'];
};


export type FullMutationInitiatePasswordResetArgs = {
  email: Scalars['String']['input'];
};


export type FullMutationInitiateUserInviteArgs = {
  input: InviteUserInput;
};


export type FullMutationResetPasswordUsingTokenArgs = {
  password: Scalars['String']['input'];
  token: Scalars['String']['input'];
};


export type FullMutationUpdateRecipientArgs = {
  input: UpdateRecipientInput;
};


export type FullMutationUpdateUserAccountArgs = {
  input: UpdateUserAccountInput;
};


export type FullMutationValidatePasswordResetTokenArgs = {
  token: Scalars['String']['input'];
};

export type FullQuery = {
  __typename: 'FullQuery';
  apiVersion: Scalars['String']['output'];
  /**
   * Retrieves a new auth bearer and refresh token
   * The refresh token is returned as a cookie
   */
  authToken: AuthTokenResponse;
  logout: LogoutResponse;
  logs: LogResponse;
  me: UserResponse;
  /** Query "recipient" entries */
  recipients: RecipientsResponse;
  /**
   * Retrieves a new auth bearer and refresh token
   * The refresh token is returned as a cookie
   */
  refreshToken: RefreshTokenResponse;
  /** Query "user_accounts" entries */
  userAccounts: UserAccountsResponse;
};


export type FullQueryAuthTokenArgs = {
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};


export type FullQueryLogsArgs = {
  filter?: InputMaybe<LogFilterInput>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<LogSortInput>>;
};


export type FullQueryRecipientsArgs = {
  filter?: InputMaybe<RecipientFilterInput>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<RecipientSortInput>>;
};


export type FullQueryUserAccountsArgs = {
  filter?: InputMaybe<UserAccountFilterInput>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<UserAccountSortInput>>;
};

export type InternalError = LogoutErrorInterface & RefreshTokenErrorInterface & {
  __typename: 'InternalError';
  description: Scalars['String']['output'];
  fullError: Scalars['String']['output'];
};

export type InvalidCredentials = AuthTokenErrorInterface & {
  __typename: 'InvalidCredentials';
  description: Scalars['String']['output'];
};

export type InvalidToken = RefreshTokenErrorInterface & {
  __typename: 'InvalidToken';
  description: Scalars['String']['output'];
};

export type InviteUserInput = {
  displayName: Scalars['String']['input'];
  email: Scalars['String']['input'];
  permissions: Array<PermissionNode>;
  username: Scalars['String']['input'];
};

export type InviteUserResponse = InviteUserResponseMessage;

export type InviteUserResponseMessage = {
  __typename: 'InviteUserResponseMessage';
  message: Scalars['String']['output'];
};

export type LogConnector = {
  __typename: 'LogConnector';
  nodes: Array<LogNode>;
  totalCount: Scalars['Int']['output'];
};

export type LogFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  recordId?: InputMaybe<EqualFilterStringInput>;
  recordType?: InputMaybe<EqualFilterLogTypeInput>;
  userId?: InputMaybe<EqualFilterStringInput>;
};

export type LogNode = {
  __typename: 'LogNode';
  datetime: Scalars['DateTime']['output'];
  id: Scalars['String']['output'];
  recordId?: Maybe<Scalars['String']['output']>;
  recordType: LogNodeType;
  user?: Maybe<UserAccountNode>;
};

export enum LogNodeType {
  RecipientCreated = 'RECIPIENT_CREATED',
  RecipientUpdated = 'RECIPIENT_UPDATED',
  UserAccountCreated = 'USER_ACCOUNT_CREATED',
  UserAccountPasswordResetInitiated = 'USER_ACCOUNT_PASSWORD_RESET_INITIATED',
  UserAccountUpdated = 'USER_ACCOUNT_UPDATED',
  UserLoggedIn = 'USER_LOGGED_IN'
}

export type LogResponse = LogConnector;

export enum LogSortFieldInput {
  Datetime = 'datetime',
  Id = 'id',
  LogType = 'logType',
  RecordId = 'recordId',
  UserId = 'userId'
}

export type LogSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: LogSortFieldInput;
};

export type Logout = {
  __typename: 'Logout';
  /** User id of the logged out user */
  userId: Scalars['String']['output'];
};

export type LogoutError = {
  __typename: 'LogoutError';
  error: LogoutErrorInterface;
};

export type LogoutErrorInterface = {
  description: Scalars['String']['output'];
};

export type LogoutResponse = Logout | LogoutError;

export type NoRefreshTokenProvided = RefreshTokenErrorInterface & {
  __typename: 'NoRefreshTokenProvided';
  description: Scalars['String']['output'];
};

export type NotARefreshToken = RefreshTokenErrorInterface & {
  __typename: 'NotARefreshToken';
  description: Scalars['String']['output'];
};

export enum NotificationTypeNode {
  Email = 'EMAIL',
  Telegram = 'TELEGRAM'
}

/**
 * Pagination input.
 *
 * Option to limit the number of returned items and/or queries large lists in "pages".
 */
export type PaginationInput = {
  /** Max number of returned items */
  first?: InputMaybe<Scalars['Int']['input']>;
  /** First returned item is at the `offset` position in the full list */
  offset?: InputMaybe<Scalars['Int']['input']>;
};

export type PasswordResetResponse = PasswordResetResponseMessage;

export type PasswordResetResponseMessage = {
  __typename: 'PasswordResetResponseMessage';
  message: Scalars['String']['output'];
};

export enum PermissionNode {
  Reader = 'READER',
  ServerAdmin = 'SERVER_ADMIN'
}

export type RecipientConnector = {
  __typename: 'RecipientConnector';
  nodes: Array<RecipientNode>;
  totalCount: Scalars['Int']['output'];
};

export type RecipientFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  notificationType?: InputMaybe<EqualFilterNotificationTypeInput>;
  search?: InputMaybe<Scalars['String']['input']>;
  toAddress?: InputMaybe<StringFilterInput>;
};

export type RecipientNode = {
  __typename: 'RecipientNode';
  auditLogs: Array<LogNode>;
  id: Scalars['String']['output'];
  name: Scalars['String']['output'];
  notificationType: NotificationTypeNode;
  toAddress: Scalars['String']['output'];
};

export enum RecipientSortFieldInput {
  Name = 'name',
  ToAddress = 'toAddress'
}

export type RecipientSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: RecipientSortFieldInput;
};

export type RecipientsResponse = RecipientConnector;

export type RefreshToken = {
  __typename: 'RefreshToken';
  /** New Bearer token */
  token: Scalars['String']['output'];
};

export type RefreshTokenError = {
  __typename: 'RefreshTokenError';
  error: RefreshTokenErrorInterface;
};

export type RefreshTokenErrorInterface = {
  description: Scalars['String']['output'];
};

export type RefreshTokenResponse = RefreshToken | RefreshTokenError;

export type SimpleStringFilterInput = {
  /** Search term must be an exact match (case sensitive) */
  equalTo?: InputMaybe<Scalars['String']['input']>;
  /** Search term must be included in search candidate (case insensitive) */
  like?: InputMaybe<Scalars['String']['input']>;
};

export type StringFilterInput = {
  endsWith?: InputMaybe<Scalars['String']['input']>;
  equalAny?: InputMaybe<Array<Scalars['String']['input']>>;
  /** Search term must be an exact match (case sensitive) */
  equalTo?: InputMaybe<Scalars['String']['input']>;
  isNull?: InputMaybe<Scalars['Boolean']['input']>;
  /** Search term must be included in search candidate (case insensitive) */
  like?: InputMaybe<Scalars['String']['input']>;
  notEqualAll?: InputMaybe<Array<Scalars['String']['input']>>;
  notEqualTo?: InputMaybe<Scalars['String']['input']>;
  startsWith?: InputMaybe<Scalars['String']['input']>;
};

export type TokenExpired = RefreshTokenErrorInterface & {
  __typename: 'TokenExpired';
  description: Scalars['String']['output'];
};

export type UpdateRecipientInput = {
  id: Scalars['String']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  toAddress?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateRecipientResponse = RecipientNode;

export type UpdateUserAccountInput = {
  displayName?: InputMaybe<Scalars['String']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  password?: InputMaybe<Scalars['String']['input']>;
  permissions?: InputMaybe<Array<PermissionNode>>;
  username?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateUserAccountResponse = UserAccountNode;

export type UserAccountConnector = {
  __typename: 'UserAccountConnector';
  nodes: Array<UserAccountNode>;
  totalCount: Scalars['Int']['output'];
};

export type UserAccountFilterInput = {
  displayName?: InputMaybe<SimpleStringFilterInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  search?: InputMaybe<Scalars['String']['input']>;
  username?: InputMaybe<SimpleStringFilterInput>;
};

export type UserAccountNode = {
  __typename: 'UserAccountNode';
  auditLogs: Array<LogNode>;
  displayName: Scalars['String']['output'];
  email?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  permissions: Array<PermissionNode>;
  username: Scalars['String']['output'];
};

export enum UserAccountSortFieldInput {
  DisplayName = 'displayName',
  Username = 'username'
}

export type UserAccountSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: UserAccountSortFieldInput;
};

export type UserAccountsResponse = UserAccountConnector;

export type UserResponse = UserAccountNode;
