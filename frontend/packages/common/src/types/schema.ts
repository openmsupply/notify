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

export type AskQuestionInput = {
  askingOrganisationId: Scalars['String']['input'];
  id: Scalars['String']['input'];
  question: Scalars['String']['input'];
  tenderRequestId: Scalars['String']['input'];
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

export type CountResponse = {
  __typename: 'CountResponse';
  count: Scalars['Int']['output'];
};

export type CreateUserAccountInput = {
  displayName?: InputMaybe<Scalars['String']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  organisationId?: InputMaybe<Scalars['String']['input']>;
  password: Scalars['String']['input'];
  permissions: Array<PermissionNode>;
  username: Scalars['String']['input'];
};

export type CreateUserAccountResponse = UserAccountNode;

export type DatabaseError = NodeErrorInterface & RefreshTokenErrorInterface & {
  __typename: 'DatabaseError';
  description: Scalars['String']['output'];
  fullError: Scalars['String']['output'];
};

export type DatetimeFilterInput = {
  afterOrEqualTo?: InputMaybe<Scalars['DateTime']['input']>;
  beforeOrEqualTo?: InputMaybe<Scalars['DateTime']['input']>;
  equalTo?: InputMaybe<Scalars['DateTime']['input']>;
};

export type DeleteGroupMemberResponse = DeleteResponse;

export type DeleteManufacturerResponse = DeleteResponse;

export type DeleteOrganisationGroupResponse = DeleteResponse;

export type DeleteOrganisationLogoResponse = DeleteResponse;

export type DeleteOrganisationResponse = DeleteResponse;

export type DeleteQuoteLinesResponse = DeleteResponse;

export type DeleteQuoteUploadResponse = DeleteResponse;

export type DeleteResponse = {
  __typename: 'DeleteResponse';
  id: Scalars['String']['output'];
};

export type DeleteTenderRequestNoticeResponse = DeleteResponse;

export type DeleteTenderRequestUploadResponse = DeleteResponse;

export type DeleteUserAccountResponse = DeleteResponse;

export type EqualFilterLogTypeInput = {
  equalAny?: InputMaybe<Array<LogNodeType>>;
  equalTo?: InputMaybe<LogNodeType>;
  notEqualTo?: InputMaybe<LogNodeType>;
};

export type EqualFilterStringInput = {
  equalAny?: InputMaybe<Array<Scalars['String']['input']>>;
  equalTo?: InputMaybe<Scalars['String']['input']>;
  notEqualTo?: InputMaybe<Scalars['String']['input']>;
};

export type EqualFilterTenderRequestStatusInput = {
  equalAny?: InputMaybe<Array<TenderRequestNodeStatus>>;
  equalTo?: InputMaybe<TenderRequestNodeStatus>;
  notEqualTo?: InputMaybe<TenderRequestNodeStatus>;
};

export type FileDownloadNode = {
  __typename: 'FileDownloadNode';
  fileId: Scalars['String']['output'];
};

export type FileDownloadResponse = FileDownloadNode;

export type FullMutation = {
  __typename: 'FullMutation';
  /** Updates user account based on a token and their information (Response to initiate_user_invite) */
  acceptUserInvite: InviteUserResponse;
  askTenderRequestQuestion: TenderRequestQuestionResponse;
  createUserAccount: CreateUserAccountResponse;
  deleteGroupMember: DeleteGroupMemberResponse;
  deleteGroupMembersForOrganisationGroupId: DeleteGroupMemberResponse;
  deleteManufacturer: DeleteManufacturerResponse;
  deleteOrganisation: DeleteOrganisationResponse;
  deleteOrganisationGroup: DeleteOrganisationGroupResponse;
  deleteOrganisationLogo: DeleteOrganisationLogoResponse;
  deleteQuoteLinesForQuoteId: DeleteQuoteLinesResponse;
  deleteQuoteUpload: DeleteQuoteUploadResponse;
  deleteTenderRequestNotice: DeleteTenderRequestNoticeResponse;
  deleteTenderRequestUpload: DeleteTenderRequestUploadResponse;
  deleteUserAccount: DeleteUserAccountResponse;
  /**
   * Initiates the password reset flow for a user based on email address
   * The user will receive an email with a link to reset their password
   */
  initiatePasswordReset: PasswordResetResponse;
  /** Invites a new user to the system */
  initiateUserInvite: InviteUserResponse;
  insertGroupMember: InsertGroupMemberResponse;
  insertManufacturer: InsertManufacturerResponse;
  insertOrganisation: InsertOrganisationResponse;
  insertOrganisationGroup: InsertOrganisationGroupResponse;
  insertQuote: InsertQuoteResponse;
  insertTenderRequest: InsertTenderRequestResponse;
  insertTenderRequestLine: InsertTenderRequestLineResponse;
  inviteSupplier: InviteResponse;
  /** Resets the password for a user based on the password reset token */
  resetPasswordUsingToken: PasswordResetResponse;
  setLateSubmissionDeadline: UpdateQuoteLateSubmissionDeadlineResponse;
  updateManufacturer: UpdateManufacturerResponse;
  updateOrganisation: UpdateOrganisationResponse;
  updateOrganisationGroup: UpdateOrganisationGroupResponse;
  updateOwnUserAccount: UpdateUserAccountResponse;
  updateQuote: UpdateQuoteResponse;
  updateTenderRequest: UpdateTenderRequestResponse;
  updateUserAccount: UpdateUserAccountResponse;
  upsertAnswer: UpsertAnswerResponse;
  upsertQuoteLine: UpsertQuoteLineResponse;
  upsertTenderRequestNotice: UpsertTenderRequestNoticeResponse;
  /** Validates Password Reset Token */
  validatePasswordResetToken: PasswordResetResponse;
};


export type FullMutationAcceptUserInviteArgs = {
  input: AcceptUserInviteInput;
  token: Scalars['String']['input'];
};


export type FullMutationAskTenderRequestQuestionArgs = {
  input: AskQuestionInput;
};


export type FullMutationCreateUserAccountArgs = {
  input: CreateUserAccountInput;
};


export type FullMutationDeleteGroupMemberArgs = {
  id: Scalars['String']['input'];
};


export type FullMutationDeleteGroupMembersForOrganisationGroupIdArgs = {
  organisationGroupId: Scalars['String']['input'];
};


export type FullMutationDeleteManufacturerArgs = {
  manufacturerId: Scalars['String']['input'];
};


export type FullMutationDeleteOrganisationArgs = {
  organisationId: Scalars['String']['input'];
};


export type FullMutationDeleteOrganisationGroupArgs = {
  id: Scalars['String']['input'];
};


export type FullMutationDeleteOrganisationLogoArgs = {
  organisationId: Scalars['String']['input'];
};


export type FullMutationDeleteQuoteLinesForQuoteIdArgs = {
  quoteId: Scalars['String']['input'];
};


export type FullMutationDeleteQuoteUploadArgs = {
  quoteUploadId: Scalars['String']['input'];
};


export type FullMutationDeleteTenderRequestNoticeArgs = {
  tenderRequestNoticeId: Scalars['String']['input'];
};


export type FullMutationDeleteTenderRequestUploadArgs = {
  tenderRequestUploadId: Scalars['String']['input'];
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


export type FullMutationInsertGroupMemberArgs = {
  input: InsertGroupMemberInput;
};


export type FullMutationInsertManufacturerArgs = {
  input: InsertManufacturerInput;
};


export type FullMutationInsertOrganisationArgs = {
  input: InsertOrganisationInput;
};


export type FullMutationInsertOrganisationGroupArgs = {
  input: InsertOrganisationGroupInput;
};


export type FullMutationInsertQuoteArgs = {
  input: InsertQuoteInput;
};


export type FullMutationInsertTenderRequestArgs = {
  input: InsertTenderRequestInput;
};


export type FullMutationInsertTenderRequestLineArgs = {
  input: InsertTenderRequestLineInput;
};


export type FullMutationInviteSupplierArgs = {
  invitedOrganisationId: Scalars['String']['input'];
  tenderRequestId: Scalars['String']['input'];
};


export type FullMutationResetPasswordUsingTokenArgs = {
  password: Scalars['String']['input'];
  token: Scalars['String']['input'];
};


export type FullMutationSetLateSubmissionDeadlineArgs = {
  input: SetLateSubmissionDeadlineInput;
  organisationId: Scalars['String']['input'];
};


export type FullMutationUpdateManufacturerArgs = {
  input: UpdateManufacturerInput;
};


export type FullMutationUpdateOrganisationArgs = {
  input: UpdateOrganisationInput;
};


export type FullMutationUpdateOrganisationGroupArgs = {
  input: UpdateOrganisationGroupInput;
};


export type FullMutationUpdateOwnUserAccountArgs = {
  input: UpdateOwnUserAccountInput;
};


export type FullMutationUpdateQuoteArgs = {
  input: UpdateQuoteInput;
};


export type FullMutationUpdateTenderRequestArgs = {
  input: UpdateTenderRequestInput;
};


export type FullMutationUpdateUserAccountArgs = {
  input: UpdateUserAccountInput;
};


export type FullMutationUpsertAnswerArgs = {
  input: UpsertAnswerInput;
};


export type FullMutationUpsertQuoteLineArgs = {
  input: UpsertQuoteLineInput;
};


export type FullMutationUpsertTenderRequestNoticeArgs = {
  input: UpsertTenderRequestNoticeInput;
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
  /** Get a single "organisation" entry */
  getOrganisation: OrganisationNode;
  groupMembers: GroupMemberResponse;
  invitedTenderRequests: TenderRequestsResponse;
  logout: LogoutResponse;
  logs: LogResponse;
  /** Query omSupply "manufacturers" entries */
  manufacturers: ManufacturersResponse;
  me: UserResponse;
  organisationGroups: OrganisationGroupResponse;
  /** Query "organisations" entries */
  organisations: OrganisationsResponse;
  /**
   * Query organisation based on matched tender request id
   * where organisation could be suppliers or manufacturers or both
   * which are participated in the mentioned tender request by id
   */
  organisationsByTenderRequest: OrganisationsByTenderRequestResponse;
  questionsByTenderRequestId: TenderRequestQuestionsResponse;
  quoteAttachmentZip: FileDownloadResponse;
  quoteDownload: FileDownloadResponse;
  quoteLines: QuoteLinesResponse;
  quoteSummary: QuoteSummariesResponse;
  quoteUploads: QuoteUploadsResponse;
  quoteUploadsByTenderRequestId: QuoteUploadsResponse;
  /** Query "quotes" entries */
  quotes: QuotesResponse;
  /**
   * Retrieves a new auth bearer and refresh token
   * The refresh token is returned as a cookie
   */
  refreshToken: RefreshTokenResponse;
  /**
   * Query "organisations" marked as suppliers
   * The list of suppliers is effectively public knowledge, where as other organisations may be private
   */
  suppliers: OrganisationsResponse;
  tenderQuoteLines: TenderQuoteLinesResponse;
  tenderRequestLines: TenderRequestLinesResponse;
  tenderRequestLinesByTenderRequestId: TenderRequestLinesResponse;
  tenderRequestNotices: TenderRequestNoticesResponse;
  tenderRequestUploads: TenderRequestUploadsResponse;
  /** Query "tender_requests" entries */
  tenderRequests: TenderRequestsResponse;
  /** Query health supply hub "user_accounts" entries */
  userAccounts: UserAccountsResponse;
};


export type FullQueryAuthTokenArgs = {
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};


export type FullQueryGetOrganisationArgs = {
  organisationId: Scalars['String']['input'];
};


export type FullQueryGroupMembersArgs = {
  filter?: InputMaybe<GroupMemberFilterInput>;
  organisationId?: InputMaybe<Scalars['String']['input']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<GroupMemberSortInput>>;
};


export type FullQueryInvitedTenderRequestsArgs = {
  filter?: InputMaybe<TenderRequestFilterInput>;
  organisationId: Scalars['String']['input'];
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<TenderRequestSortInput>>;
};


export type FullQueryLogsArgs = {
  filter?: InputMaybe<LogFilterInput>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<LogSortInput>>;
};


export type FullQueryManufacturersArgs = {
  filter?: InputMaybe<ManufacturerFilterInput>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<ManufacturerSortInput>>;
};


export type FullQueryOrganisationGroupsArgs = {
  filter?: InputMaybe<OrganisationGroupFilterInput>;
  organisationId?: InputMaybe<Scalars['String']['input']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<OrganisationGroupSortInput>>;
};


export type FullQueryOrganisationsArgs = {
  filter?: InputMaybe<OrganisationFilterInput>;
  organisationId?: InputMaybe<Scalars['String']['input']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<OrganisationSortInput>>;
};


export type FullQueryOrganisationsByTenderRequestArgs = {
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryQuestionsByTenderRequestIdArgs = {
  filter?: InputMaybe<TenderQuestionFilterInput>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<TenderQuestionSortInput>>;
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryQuoteAttachmentZipArgs = {
  organisationId?: InputMaybe<Scalars['String']['input']>;
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryQuoteDownloadArgs = {
  organisationId?: InputMaybe<Scalars['String']['input']>;
  quoteId: Scalars['String']['input'];
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryQuoteLinesArgs = {
  filter?: InputMaybe<QuoteLineFilterInput>;
  organisationId?: InputMaybe<Scalars['String']['input']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<QuoteLineSortInput>>;
};


export type FullQueryQuoteSummaryArgs = {
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryQuoteUploadsArgs = {
  organisationId?: InputMaybe<Scalars['String']['input']>;
  quoteId: Scalars['String']['input'];
};


export type FullQueryQuoteUploadsByTenderRequestIdArgs = {
  organisationId?: InputMaybe<Scalars['String']['input']>;
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryQuotesArgs = {
  filter?: InputMaybe<QuoteFilterInput>;
  organisationId?: InputMaybe<Scalars['String']['input']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<QuoteSortInput>>;
};


export type FullQuerySuppliersArgs = {
  filter?: InputMaybe<OrganisationFilterInput>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<OrganisationSortInput>>;
};


export type FullQueryTenderQuoteLinesArgs = {
  filter?: InputMaybe<TenderQuoteLineFilterInput>;
  organisationId: Scalars['String']['input'];
  quoteId: Scalars['String']['input'];
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryTenderRequestLinesArgs = {
  filter?: InputMaybe<TenderRequestLineFilterInput>;
  organisationId: Scalars['String']['input'];
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<TenderRequestLineSortInput>>;
};


export type FullQueryTenderRequestLinesByTenderRequestIdArgs = {
  organisationId: Scalars['String']['input'];
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryTenderRequestNoticesArgs = {
  organisationId?: InputMaybe<Scalars['String']['input']>;
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryTenderRequestUploadsArgs = {
  organisationId?: InputMaybe<Scalars['String']['input']>;
  tenderRequestId: Scalars['String']['input'];
};


export type FullQueryTenderRequestsArgs = {
  filter?: InputMaybe<TenderRequestFilterInput>;
  organisationId?: InputMaybe<Scalars['String']['input']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<TenderRequestSortInput>>;
};


export type FullQueryUserAccountsArgs = {
  filter?: InputMaybe<UserAccountFilterInput>;
  organisationId?: InputMaybe<Scalars['String']['input']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<UserAccountSortInput>>;
};

export type GroupMemberConnector = {
  __typename: 'GroupMemberConnector';
  nodes: Array<GroupMemberNode>;
  totalCount: Scalars['Int']['output'];
};

export type GroupMemberFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  organisationGroupId?: InputMaybe<EqualFilterStringInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
  organisationName?: InputMaybe<StringFilterInput>;
};

export type GroupMemberNode = {
  __typename: 'GroupMemberNode';
  id: Scalars['String']['output'];
  organisation?: Maybe<OrganisationNode>;
  organisationGroupId: Scalars['String']['output'];
  organisationId: Scalars['String']['output'];
};

export type GroupMemberResponse = GroupMemberConnector;

export enum GroupMemberSortFieldInput {
  Name = 'name'
}

export type GroupMemberSortInput = {
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  key: GroupMemberSortFieldInput;
};

export type IdResponse = {
  __typename: 'IdResponse';
  id: Scalars['String']['output'];
};

export type InsertGroupMemberInput = {
  id: Scalars['String']['input'];
  organisationGroupId: Scalars['String']['input'];
  organisationId: Scalars['String']['input'];
};

export type InsertGroupMemberResponse = GroupMemberNode;

export type InsertManufacturerInput = {
  code: Scalars['String']['input'];
  country?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  name: Scalars['String']['input'];
};

export type InsertManufacturerResponse = ManufacturerNode;

export type InsertOrganisationGroupInput = {
  id: Scalars['String']['input'];
  name: Scalars['String']['input'];
  organisationId: Scalars['String']['input'];
};

export type InsertOrganisationGroupResponse = OrganisationGroupNode;

export type InsertOrganisationInput = {
  code: Scalars['String']['input'];
  contactName?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  isCustomer: Scalars['Boolean']['input'];
  isManufacturer: Scalars['Boolean']['input'];
  isSupplier: Scalars['Boolean']['input'];
  name: Scalars['String']['input'];
  phone?: InputMaybe<Scalars['String']['input']>;
  website?: InputMaybe<Scalars['String']['input']>;
};

export type InsertOrganisationResponse = OrganisationNode;

export type InsertQuoteInput = {
  currency: Scalars['String']['input'];
  freightType: Scalars['String']['input'];
  id: Scalars['String']['input'];
  organisationId: Scalars['String']['input'];
  statusReason: Scalars['String']['input'];
  supplierNotes?: InputMaybe<Scalars['String']['input']>;
  tenderRequestId: Scalars['String']['input'];
};

export type InsertQuoteResponse = QuoteNode;

export type InsertTenderRequestInput = {
  closingDatetime: Scalars['DateTime']['input'];
  description: Scalars['String']['input'];
  id: Scalars['String']['input'];
  incoterm?: InputMaybe<Scalars['String']['input']>;
  organisationId: Scalars['String']['input'];
  publishedDatetime?: InputMaybe<Scalars['DateTime']['input']>;
};

export type InsertTenderRequestLineInput = {
  comments: Scalars['String']['input'];
  conditions: Scalars['String']['input'];
  id: Scalars['String']['input'];
  itemCode: Scalars['String']['input'];
  itemId: Scalars['String']['input'];
  itemName: Scalars['String']['input'];
  msupplyLineNumber: Scalars['Int']['input'];
  numberOfPacks: Scalars['Float']['input'];
  preferredPackSize: Scalars['Float']['input'];
  productSpecifications?: InputMaybe<Scalars['String']['input']>;
  tenderRequestId: Scalars['String']['input'];
  unitQuantity: Scalars['Float']['input'];
  units: Scalars['String']['input'];
  universalCode?: InputMaybe<Scalars['String']['input']>;
};

export type InsertTenderRequestLineResponse = TenderRequestLineNode;

export type InsertTenderRequestResponse = TenderRequestNode;

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

export type InviteNode = {
  __typename: 'InviteNode';
  id: Scalars['String']['output'];
  invitedOrganisationId: Scalars['String']['output'];
  lateSubmissionDeadline?: Maybe<Scalars['DateTime']['output']>;
  lateSubmissionReason?: Maybe<Scalars['String']['output']>;
  tenderRequestId: Scalars['String']['output'];
};

export type InviteResponse = IdResponse;

export type InviteUserInput = {
  displayName: Scalars['String']['input'];
  email: Scalars['String']['input'];
  organisationId: Scalars['String']['input'];
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
  organisationId?: InputMaybe<EqualFilterStringInput>;
  recordId?: InputMaybe<EqualFilterStringInput>;
  recordType?: InputMaybe<EqualFilterLogTypeInput>;
  userId?: InputMaybe<EqualFilterStringInput>;
};

export type LogNode = {
  __typename: 'LogNode';
  datetime: Scalars['DateTime']['output'];
  id: Scalars['String']['output'];
  organisation?: Maybe<OrganisationNode>;
  organisationId?: Maybe<Scalars['String']['output']>;
  recordId?: Maybe<Scalars['String']['output']>;
  recordType: LogNodeType;
  user?: Maybe<UserAccountNode>;
};

export enum LogNodeType {
  LateQuotesAllowed = 'LATE_QUOTES_ALLOWED',
  ManufacturerCreated = 'MANUFACTURER_CREATED',
  ManufacturerUpdated = 'MANUFACTURER_UPDATED',
  NoticeModified = 'NOTICE_MODIFIED',
  OrganisationCreated = 'ORGANISATION_CREATED',
  OrganisationUpdated = 'ORGANISATION_UPDATED',
  QuoteDocumentUploaded = 'QUOTE_DOCUMENT_UPLOADED',
  QuoteUpdated = 'QUOTE_UPDATED',
  TenderCreated = 'TENDER_CREATED',
  TenderInvited = 'TENDER_INVITED',
  TenderRequestUploaded = 'TENDER_REQUEST_UPLOADED',
  TenderUpdated = 'TENDER_UPDATED',
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

export type ManufacturerConnector = {
  __typename: 'ManufacturerConnector';
  nodes: Array<ManufacturerNode>;
  totalCount: Scalars['Int']['output'];
};

export type ManufacturerFilterInput = {
  country?: InputMaybe<SimpleStringFilterInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  name?: InputMaybe<SimpleStringFilterInput>;
};

export type ManufacturerNode = {
  __typename: 'ManufacturerNode';
  auditLogs: Array<LogNode>;
  code: Scalars['String']['output'];
  country?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  name: Scalars['String']['output'];
};

export enum ManufacturerSortFieldInput {
  Code = 'code',
  Country = 'country',
  Name = 'name'
}

export type ManufacturerSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: ManufacturerSortFieldInput;
};

export type ManufacturersResponse = ManufacturerConnector;

export type NoRefreshTokenProvided = RefreshTokenErrorInterface & {
  __typename: 'NoRefreshTokenProvided';
  description: Scalars['String']['output'];
};

/** Generic Error Wrapper */
export type NodeError = {
  __typename: 'NodeError';
  error: NodeErrorInterface;
};

export type NodeErrorInterface = {
  description: Scalars['String']['output'];
};

export type NotARefreshToken = RefreshTokenErrorInterface & {
  __typename: 'NotARefreshToken';
  description: Scalars['String']['output'];
};

export type OrganisationConnector = {
  __typename: 'OrganisationConnector';
  nodes: Array<OrganisationNode>;
  totalCount: Scalars['Int']['output'];
};

export type OrganisationFilterInput = {
  code?: InputMaybe<StringFilterInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  isCustomer?: InputMaybe<Scalars['Boolean']['input']>;
  isManufacturer?: InputMaybe<Scalars['Boolean']['input']>;
  isSupplier?: InputMaybe<Scalars['Boolean']['input']>;
  modifiedBy?: InputMaybe<EqualFilterStringInput>;
  name?: InputMaybe<StringFilterInput>;
  search?: InputMaybe<Scalars['String']['input']>;
};

export type OrganisationGroupConnector = {
  __typename: 'OrganisationGroupConnector';
  nodes: Array<OrganisationGroupNode>;
  totalCount: Scalars['Int']['output'];
};

export type OrganisationGroupFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  name?: InputMaybe<SimpleStringFilterInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
};

export type OrganisationGroupNode = {
  __typename: 'OrganisationGroupNode';
  id: Scalars['String']['output'];
  name: Scalars['String']['output'];
  organisationId: Scalars['String']['output'];
};

export type OrganisationGroupResponse = OrganisationGroupConnector;

export enum OrganisationGroupSortFieldInput {
  Name = 'name'
}

export type OrganisationGroupSortInput = {
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  key: OrganisationGroupSortFieldInput;
};

export type OrganisationNode = {
  __typename: 'OrganisationNode';
  auditLogs: Array<LogNode>;
  code: Scalars['String']['output'];
  contactName?: Maybe<Scalars['String']['output']>;
  country?: Maybe<Scalars['String']['output']>;
  createdDatetime?: Maybe<Scalars['DateTime']['output']>;
  description?: Maybe<Scalars['String']['output']>;
  discoverable: Scalars['Boolean']['output'];
  email?: Maybe<Scalars['String']['output']>;
  entityType?: Maybe<Scalars['String']['output']>;
  fullLegalName?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  isCustomer: Scalars['Boolean']['output'];
  isManufacturer: Scalars['Boolean']['output'];
  isSupplier: Scalars['Boolean']['output'];
  logoId?: Maybe<Scalars['String']['output']>;
  modifiedBy: Scalars['String']['output'];
  modifiedDatetime?: Maybe<Scalars['DateTime']['output']>;
  name: Scalars['String']['output'];
  operatingAddress?: Maybe<Scalars['String']['output']>;
  phone?: Maybe<Scalars['String']['output']>;
  postalAddress?: Maybe<Scalars['String']['output']>;
  registrationJurisdiction?: Maybe<Scalars['String']['output']>;
  registrationNumber?: Maybe<Scalars['String']['output']>;
  website?: Maybe<Scalars['String']['output']>;
};

export enum OrganisationSortFieldInput {
  Code = 'code',
  ContactName = 'contactName',
  Name = 'name'
}

export type OrganisationSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: OrganisationSortFieldInput;
};

export type OrganisationsByTenderRequestNode = {
  __typename: 'OrganisationsByTenderRequestNode';
  /** Selected manufacturers for this tender request */
  manufacturers: OrganisationConnector;
  /** Invited suppliers for this tender request */
  suppliers: OrganisationConnector;
};

export type OrganisationsByTenderRequestResponse = NodeError | OrganisationsByTenderRequestNode;

export type OrganisationsResponse = OrganisationConnector;

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
  OrganisationAccess = 'ORGANISATION_ACCESS',
  OrganisationAdmin = 'ORGANISATION_ADMIN',
  OrganisationIntegration = 'ORGANISATION_INTEGRATION',
  ServerAdmin = 'SERVER_ADMIN'
}

export type QuoteConnector = {
  __typename: 'QuoteConnector';
  nodes: Array<QuoteNode>;
  totalCount: Scalars['Int']['output'];
};

export type QuoteFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
  tenderRequestId?: InputMaybe<EqualFilterStringInput>;
};

export type QuoteLineConnector = {
  __typename: 'QuoteLineConnector';
  nodes: Array<QuoteLineNode>;
  totalCount: Scalars['Int']['output'];
};

export type QuoteLineFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  quoteId?: InputMaybe<EqualFilterStringInput>;
};

export type QuoteLineNode = {
  __typename: 'QuoteLineNode';
  createdDatetime: Scalars['DateTime']['output'];
  currency: Scalars['String']['output'];
  deliveryLeadTime: Scalars['String']['output'];
  expiryPeriod: Scalars['String']['output'];
  id: Scalars['String']['output'];
  itemCode: Scalars['String']['output'];
  manufacturerId: Scalars['String']['output'];
  methodOfDelivery: Scalars['String']['output'];
  modifiedDatetime: Scalars['DateTime']['output'];
  numberOfPacks: Scalars['Int']['output'];
  packSize: Scalars['Float']['output'];
  pricePerPack: Scalars['Float']['output'];
  quoteId: Scalars['String']['output'];
  supplierItemId?: Maybe<Scalars['String']['output']>;
  supplierNotes: Scalars['String']['output'];
  tenderRequestLineId: Scalars['String']['output'];
};

export enum QuoteLineSortFieldInput {
  CreatedDatetime = 'createdDatetime'
}

export type QuoteLineSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: QuoteLineSortFieldInput;
};

export type QuoteLinesResponse = QuoteLineConnector;

export type QuoteNode = {
  __typename: 'QuoteNode';
  auditLogs: Array<LogNode>;
  createdDatetime: Scalars['DateTime']['output'];
  currency: Scalars['String']['output'];
  customerNotes?: Maybe<Scalars['String']['output']>;
  finalisedDatetime?: Maybe<Scalars['DateTime']['output']>;
  freightType: Scalars['String']['output'];
  id: Scalars['String']['output'];
  invite?: Maybe<InviteNode>;
  isLate: Scalars['Boolean']['output'];
  lineCount: Scalars['Int']['output'];
  modifiedDatetime: Scalars['DateTime']['output'];
  organisation?: Maybe<OrganisationNode>;
  organisationId: Scalars['String']['output'];
  status: QuoteNodeStatus;
  statusReason: Scalars['String']['output'];
  supplierNotes?: Maybe<Scalars['String']['output']>;
  tenderRequestId: Scalars['String']['output'];
};

export enum QuoteNodeStatus {
  Confirmed = 'CONFIRMED',
  Draft = 'DRAFT',
  Withdrawn = 'WITHDRAWN'
}

export enum QuoteSortFieldInput {
  CreatedDatetime = 'createdDatetime',
  Status = 'status'
}

export type QuoteSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: QuoteSortFieldInput;
};

export type QuoteSummariesResponse = QuoteSummaryConnector;

export type QuoteSummaryConnector = {
  __typename: 'QuoteSummaryConnector';
  nodes: Array<QuoteSummaryNode>;
  totalCount: Scalars['Int']['output'];
};

export type QuoteSummaryNode = {
  __typename: 'QuoteSummaryNode';
  id: Scalars['String']['output'];
  invite?: Maybe<InviteNode>;
  lineCount: Scalars['Int']['output'];
  modifiedDatetime: Scalars['DateTime']['output'];
  organisation?: Maybe<OrganisationNode>;
  organisationId: Scalars['String']['output'];
  status: QuoteSummaryNodeStatus;
  statusReason: Scalars['String']['output'];
  tenderRequestId: Scalars['String']['output'];
};

export enum QuoteSummaryNodeStatus {
  Confirmed = 'CONFIRMED',
  ConfirmedLate = 'CONFIRMED_LATE',
  Draft = 'DRAFT',
  Invited = 'INVITED',
  Withdrawn = 'WITHDRAWN'
}

export type QuoteUploadConnector = {
  __typename: 'QuoteUploadConnector';
  nodes: Array<QuoteUploadNode>;
  totalCount: Scalars['Int']['output'];
};

export type QuoteUploadNode = {
  __typename: 'QuoteUploadNode';
  contentType: Scalars['String']['output'];
  createdDatetime: Scalars['DateTime']['output'];
  description: Scalars['String']['output'];
  fileName: Scalars['String']['output'];
  id: Scalars['String']['output'];
  modifiedDatetime: Scalars['DateTime']['output'];
  organisation?: Maybe<OrganisationNode>;
  organisationId?: Maybe<Scalars['String']['output']>;
  quoteId: Scalars['String']['output'];
};

export type QuoteUploadsResponse = QuoteUploadConnector;

export type QuotesResponse = QuoteConnector;

export type RecordNotFound = NodeErrorInterface & {
  __typename: 'RecordNotFound';
  description: Scalars['String']['output'];
};

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

export type SetLateSubmissionDeadlineInput = {
  lateSubmissionDeadline: Scalars['DateTime']['input'];
  organisationIds: Array<Scalars['String']['input']>;
  reason: Scalars['String']['input'];
  tenderRequestId: Scalars['String']['input'];
};

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

export type TenderQuestionFilterInput = {
  answer?: InputMaybe<EqualFilterStringInput>;
  askingOrganisation?: InputMaybe<EqualFilterStringInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  tenderRequestId?: InputMaybe<EqualFilterStringInput>;
};

export enum TenderQuestionSortFieldInput {
  Answer = 'answer',
  AnsweredDatetime = 'answeredDatetime',
  CreatedDatetime = 'createdDatetime',
  ModifiedDatetime = 'modifiedDatetime'
}

export type TenderQuestionSortInput = {
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  key: TenderQuestionSortFieldInput;
};

export type TenderQuoteLineConnector = {
  __typename: 'TenderQuoteLineConnector';
  nodes: Array<TenderQuoteLineNode>;
  totalCount: Scalars['Int']['output'];
};

export type TenderQuoteLineFilterInput = {
  isQuoted?: InputMaybe<Scalars['Boolean']['input']>;
  itemName?: InputMaybe<StringFilterInput>;
};

export type TenderQuoteLineNode = {
  __typename: 'TenderQuoteLineNode';
  conditions: Scalars['String']['output'];
  currency: Scalars['String']['output'];
  id: Scalars['String']['output'];
  itemName: Scalars['String']['output'];
  pricePerPack: Scalars['Float']['output'];
  productSpecifications: Scalars['String']['output'];
  quoteLineId: Scalars['String']['output'];
  quotedNumberOfPacks: Scalars['Int']['output'];
  requestedNumberOfPacks: Scalars['Float']['output'];
  supplierNotes: Scalars['String']['output'];
  tenderRequestId: Scalars['String']['output'];
  totalPrice: Scalars['Float']['output'];
  unitQuantity: Scalars['Float']['output'];
};

export type TenderQuoteLinesResponse = TenderQuoteLineConnector;

export type TenderRequestConnector = {
  __typename: 'TenderRequestConnector';
  nodes: Array<TenderRequestNode>;
  totalCount: Scalars['Int']['output'];
};

export type TenderRequestFilterInput = {
  closingDatetime?: InputMaybe<DatetimeFilterInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
  status?: InputMaybe<EqualFilterTenderRequestStatusInput>;
};

export type TenderRequestLineConnector = {
  __typename: 'TenderRequestLineConnector';
  nodes: Array<TenderRequestLineNode>;
  totalCount: Scalars['Int']['output'];
};

export type TenderRequestLineFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  itemName?: InputMaybe<StringFilterInput>;
  tenderRequestId?: InputMaybe<EqualFilterStringInput>;
};

export type TenderRequestLineNode = {
  __typename: 'TenderRequestLineNode';
  comments: Scalars['String']['output'];
  conditions: Scalars['String']['output'];
  id: Scalars['String']['output'];
  itemCode: Scalars['String']['output'];
  itemId: Scalars['String']['output'];
  itemName: Scalars['String']['output'];
  numberOfPacks: Scalars['Float']['output'];
  preferredPackSize: Scalars['Float']['output'];
  productSpecifications?: Maybe<Scalars['String']['output']>;
  tenderRequestId: Scalars['String']['output'];
  unitQuantity: Scalars['Float']['output'];
  universalCode?: Maybe<Scalars['String']['output']>;
};

export enum TenderRequestLineSortFieldInput {
  Conditions = 'conditions',
  Id = 'id',
  ItemName = 'itemName',
  LineNumber = 'lineNumber',
  ProductSpecifications = 'productSpecifications'
}

export type TenderRequestLineSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: TenderRequestLineSortFieldInput;
};

export type TenderRequestLinesResponse = TenderRequestLineConnector;

export type TenderRequestNode = {
  __typename: 'TenderRequestNode';
  auditLogs: Array<LogNode>;
  closingDatetime: Scalars['DateTime']['output'];
  createdDatetime: Scalars['DateTime']['output'];
  description: Scalars['String']['output'];
  id: Scalars['String']['output'];
  incoterm?: Maybe<Scalars['String']['output']>;
  lineCount: Scalars['Int']['output'];
  modifiedDatetime: Scalars['DateTime']['output'];
  organisation?: Maybe<OrganisationNode>;
  organisationId: Scalars['String']['output'];
  publishedDatetime?: Maybe<Scalars['DateTime']['output']>;
  reason?: Maybe<Scalars['String']['output']>;
  status: TenderRequestNodeStatus;
};

export enum TenderRequestNodeStatus {
  Cancelled = 'CANCELLED',
  Closed = 'CLOSED',
  Draft = 'DRAFT',
  Open = 'OPEN',
  Withdrawn = 'WITHDRAWN'
}

export type TenderRequestNoticeConnector = {
  __typename: 'TenderRequestNoticeConnector';
  nodes: Array<TenderRequestNoticeNode>;
  totalCount: Scalars['Int']['output'];
};

export type TenderRequestNoticeNode = {
  __typename: 'TenderRequestNoticeNode';
  body: Scalars['String']['output'];
  createdBy: Scalars['String']['output'];
  createdDatetime: Scalars['DateTime']['output'];
  id: Scalars['String']['output'];
  modifiedBy: Scalars['String']['output'];
  modifiedDatetime: Scalars['DateTime']['output'];
  originalId: Scalars['String']['output'];
  tenderRequestId: Scalars['String']['output'];
  title: Scalars['String']['output'];
};

export type TenderRequestNoticesResponse = TenderRequestNoticeConnector;

export type TenderRequestQuestionConnector = {
  __typename: 'TenderRequestQuestionConnector';
  nodes: Array<TenderRequestQuestionNode>;
  totalCount: Scalars['Int']['output'];
};

export type TenderRequestQuestionNode = {
  __typename: 'TenderRequestQuestionNode';
  answer?: Maybe<Scalars['String']['output']>;
  answeredBy?: Maybe<Scalars['String']['output']>;
  answeredDatetime?: Maybe<Scalars['DateTime']['output']>;
  askingOrganisation: Scalars['String']['output'];
  createdBy: Scalars['String']['output'];
  createdDatetime: Scalars['DateTime']['output'];
  id: Scalars['String']['output'];
  modifiedBy: Scalars['String']['output'];
  modifiedDatetime: Scalars['DateTime']['output'];
  noticeId?: Maybe<Scalars['String']['output']>;
  question: Scalars['String']['output'];
  tenderRequestId: Scalars['String']['output'];
};

export type TenderRequestQuestionResponse = NodeError | TenderRequestQuestionNode;

export type TenderRequestQuestionsResponse = TenderRequestQuestionConnector;

export enum TenderRequestSortFieldInput {
  ClosingDatetime = 'closingDatetime',
  Status = 'status'
}

export type TenderRequestSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']['input']>;
  /** Sort query result by `key` */
  key: TenderRequestSortFieldInput;
};

export type TenderRequestUploadConnector = {
  __typename: 'TenderRequestUploadConnector';
  nodes: Array<TenderRequestUploadNode>;
  totalCount: Scalars['Int']['output'];
};

export type TenderRequestUploadNode = {
  __typename: 'TenderRequestUploadNode';
  contentType: Scalars['String']['output'];
  createdDatetime: Scalars['DateTime']['output'];
  description: Scalars['String']['output'];
  fileName: Scalars['String']['output'];
  id: Scalars['String']['output'];
  modifiedDatetime: Scalars['DateTime']['output'];
  tenderRequestId: Scalars['String']['output'];
};

export type TenderRequestUploadsResponse = TenderRequestUploadConnector;

export type TenderRequestsResponse = TenderRequestConnector;

export type TokenExpired = RefreshTokenErrorInterface & {
  __typename: 'TokenExpired';
  description: Scalars['String']['output'];
};

export type UpdateManufacturerInput = {
  code?: InputMaybe<Scalars['String']['input']>;
  country?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateManufacturerResponse = ManufacturerNode;

export type UpdateOrganisationGroupInput = {
  id: Scalars['String']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  organisationId?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateOrganisationGroupResponse = OrganisationGroupNode;

export type UpdateOrganisationInput = {
  code?: InputMaybe<Scalars['String']['input']>;
  contactName?: InputMaybe<Scalars['String']['input']>;
  country?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  discoverable?: InputMaybe<Scalars['Boolean']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  entityType?: InputMaybe<Scalars['String']['input']>;
  fullLegalName?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  isCustomer?: InputMaybe<Scalars['Boolean']['input']>;
  isManufacturer?: InputMaybe<Scalars['Boolean']['input']>;
  isSupplier?: InputMaybe<Scalars['Boolean']['input']>;
  logoId?: InputMaybe<Scalars['String']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
  operatingAddress?: InputMaybe<Scalars['String']['input']>;
  phone?: InputMaybe<Scalars['String']['input']>;
  postalAddress?: InputMaybe<Scalars['String']['input']>;
  registrationJurisdiction?: InputMaybe<Scalars['String']['input']>;
  registrationNumber?: InputMaybe<Scalars['String']['input']>;
  website?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateOrganisationResponse = OrganisationNode;

export type UpdateOwnUserAccountInput = {
  displayName?: InputMaybe<Scalars['String']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  password?: InputMaybe<Scalars['String']['input']>;
  username?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateQuoteInput = {
  id: Scalars['String']['input'];
  status?: InputMaybe<QuoteNodeStatus>;
  statusReason?: InputMaybe<Scalars['String']['input']>;
  supplierNotes?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateQuoteLateSubmissionDeadlineResponse = CountResponse;

export type UpdateQuoteResponse = QuoteNode;

export type UpdateTenderRequestInput = {
  closingDatetime?: InputMaybe<Scalars['DateTime']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  reason?: InputMaybe<Scalars['String']['input']>;
  status?: InputMaybe<TenderRequestNodeStatus>;
};

export type UpdateTenderRequestResponse = TenderRequestNode;

export type UpdateUserAccountInput = {
  displayName?: InputMaybe<Scalars['String']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['String']['input'];
  password?: InputMaybe<Scalars['String']['input']>;
  permissions?: InputMaybe<Array<PermissionNode>>;
  username?: InputMaybe<Scalars['String']['input']>;
};

export type UpdateUserAccountResponse = UserAccountNode;

export type UpsertAnswerInput = {
  answer: Scalars['String']['input'];
  id: Scalars['String']['input'];
  noticeId?: InputMaybe<Scalars['String']['input']>;
  tenderRequestId: Scalars['String']['input'];
};

export type UpsertAnswerResponse = NodeError | TenderRequestQuestionNode;

export type UpsertQuoteLineInput = {
  comments: Scalars['String']['input'];
  currency: Scalars['String']['input'];
  deliveryLeadTime: Scalars['String']['input'];
  expiryPeriod: Scalars['String']['input'];
  id: Scalars['String']['input'];
  itemCode: Scalars['String']['input'];
  itemName: Scalars['String']['input'];
  manufacturerName: Scalars['String']['input'];
  methodOfDelivery: Scalars['String']['input'];
  numberOfPacks: Scalars['Int']['input'];
  packSize: Scalars['Float']['input'];
  pricePerPack: Scalars['Float']['input'];
  quoteId: Scalars['String']['input'];
  supplierItemId?: InputMaybe<Scalars['String']['input']>;
  tenderRequestLineId: Scalars['String']['input'];
};

export type UpsertQuoteLineResponse = QuoteLineNode;

export type UpsertTenderRequestNoticeInput = {
  body: Scalars['String']['input'];
  id: Scalars['String']['input'];
  originalId: Scalars['String']['input'];
  tenderRequestId: Scalars['String']['input'];
  title: Scalars['String']['input'];
};

export type UpsertTenderRequestNoticeResponse = TenderRequestNoticeNode;

export type UserAccountConnector = {
  __typename: 'UserAccountConnector';
  nodes: Array<UserAccountNode>;
  totalCount: Scalars['Int']['output'];
};

export type UserAccountFilterInput = {
  displayName?: InputMaybe<SimpleStringFilterInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
  search?: InputMaybe<Scalars['String']['input']>;
  username?: InputMaybe<SimpleStringFilterInput>;
};

export type UserAccountNode = {
  __typename: 'UserAccountNode';
  auditLogs: Array<LogNode>;
  displayName: Scalars['String']['output'];
  email?: Maybe<Scalars['String']['output']>;
  id: Scalars['String']['output'];
  organisation?: Maybe<OrganisationNode>;
  organisationId?: Maybe<Scalars['String']['output']>;
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
