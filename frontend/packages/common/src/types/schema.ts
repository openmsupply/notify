export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  /**
   * Implement the DateTime<Utc> scalar
   *
   * The input/output is a string in RFC3339 format.
   */
  DateTime: string;
};

export type AcceptUserInviteInput = {
  displayName: Scalars['String'];
  password: Scalars['String'];
  username: Scalars['String'];
};

export type AccessDenied = LogoutErrorInterface & {
  __typename: 'AccessDenied';
  description: Scalars['String'];
  fullError: Scalars['String'];
};

export type AskQuestionInput = {
  askingOrganisationId: Scalars['String'];
  id: Scalars['String'];
  question: Scalars['String'];
  tenderRequestId: Scalars['String'];
};

export type AuthToken = {
  __typename: 'AuthToken';
  /** Bearer token */
  token: Scalars['String'];
};

export type AuthTokenError = {
  __typename: 'AuthTokenError';
  error: AuthTokenErrorInterface;
};

export type AuthTokenErrorInterface = {
  description: Scalars['String'];
};

export type AuthTokenResponse = AuthToken | AuthTokenError;

export type CreateUserAccountInput = {
  displayName?: InputMaybe<Scalars['String']>;
  email?: InputMaybe<Scalars['String']>;
  id: Scalars['String'];
  organisationId?: InputMaybe<Scalars['String']>;
  password: Scalars['String'];
  permissions: Array<PermissionNode>;
  username: Scalars['String'];
};

export type CreateUserAccountResponse = UserAccountNode;

export type DatabaseError = NodeErrorInterface & RefreshTokenErrorInterface & {
  __typename: 'DatabaseError';
  description: Scalars['String'];
  fullError: Scalars['String'];
};

export type DatetimeFilterInput = {
  afterOrEqualTo?: InputMaybe<Scalars['DateTime']>;
  beforeOrEqualTo?: InputMaybe<Scalars['DateTime']>;
  equalTo?: InputMaybe<Scalars['DateTime']>;
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
  id: Scalars['String'];
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
  equalAny?: InputMaybe<Array<Scalars['String']>>;
  equalTo?: InputMaybe<Scalars['String']>;
  notEqualTo?: InputMaybe<Scalars['String']>;
};

export type EqualFilterTenderRequestStatusInput = {
  equalAny?: InputMaybe<Array<TenderRequestNodeStatus>>;
  equalTo?: InputMaybe<TenderRequestNodeStatus>;
  notEqualTo?: InputMaybe<TenderRequestNodeStatus>;
};

export type FileDownloadNode = {
  __typename: 'FileDownloadNode';
  fileId: Scalars['String'];
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
  token: Scalars['String'];
};


export type FullMutationAskTenderRequestQuestionArgs = {
  input: AskQuestionInput;
};


export type FullMutationCreateUserAccountArgs = {
  input: CreateUserAccountInput;
};


export type FullMutationDeleteGroupMemberArgs = {
  id: Scalars['String'];
};


export type FullMutationDeleteGroupMembersForOrganisationGroupIdArgs = {
  organisationGroupId: Scalars['String'];
};


export type FullMutationDeleteManufacturerArgs = {
  manufacturerId: Scalars['String'];
};


export type FullMutationDeleteOrganisationArgs = {
  organisationId: Scalars['String'];
};


export type FullMutationDeleteOrganisationGroupArgs = {
  id: Scalars['String'];
};


export type FullMutationDeleteOrganisationLogoArgs = {
  organisationId: Scalars['String'];
};


export type FullMutationDeleteQuoteLinesForQuoteIdArgs = {
  quoteId: Scalars['String'];
};


export type FullMutationDeleteQuoteUploadArgs = {
  quoteUploadId: Scalars['String'];
};


export type FullMutationDeleteTenderRequestNoticeArgs = {
  tenderRequestNoticeId: Scalars['String'];
};


export type FullMutationDeleteTenderRequestUploadArgs = {
  tenderRequestUploadId: Scalars['String'];
};


export type FullMutationDeleteUserAccountArgs = {
  userAccountId: Scalars['String'];
};


export type FullMutationInitiatePasswordResetArgs = {
  email: Scalars['String'];
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
  invitedOrganisationId: Scalars['String'];
  tenderRequestId: Scalars['String'];
};


export type FullMutationResetPasswordUsingTokenArgs = {
  password: Scalars['String'];
  token: Scalars['String'];
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
  token: Scalars['String'];
};

export type FullQuery = {
  __typename: 'FullQuery';
  apiVersion: Scalars['String'];
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
  password: Scalars['String'];
  username: Scalars['String'];
};


export type FullQueryGetOrganisationArgs = {
  organisationId: Scalars['String'];
};


export type FullQueryGroupMembersArgs = {
  filter?: InputMaybe<GroupMemberFilterInput>;
  organisationId?: InputMaybe<Scalars['String']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<GroupMemberSortInput>>;
};


export type FullQueryInvitedTenderRequestsArgs = {
  filter?: InputMaybe<TenderRequestFilterInput>;
  organisationId: Scalars['String'];
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
  organisationId?: InputMaybe<Scalars['String']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<OrganisationGroupSortInput>>;
};


export type FullQueryOrganisationsArgs = {
  filter?: InputMaybe<OrganisationFilterInput>;
  organisationId?: InputMaybe<Scalars['String']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<OrganisationSortInput>>;
};


export type FullQueryOrganisationsByTenderRequestArgs = {
  tenderRequestId: Scalars['String'];
};


export type FullQueryQuestionsByTenderRequestIdArgs = {
  filter?: InputMaybe<TenderQuestionFilterInput>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<TenderQuestionSortInput>>;
  tenderRequestId: Scalars['String'];
};


export type FullQueryQuoteAttachmentZipArgs = {
  organisationId?: InputMaybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
};


export type FullQueryQuoteDownloadArgs = {
  organisationId?: InputMaybe<Scalars['String']>;
  quoteId: Scalars['String'];
  tenderRequestId: Scalars['String'];
};


export type FullQueryQuoteLinesArgs = {
  filter?: InputMaybe<QuoteLineFilterInput>;
  organisationId?: InputMaybe<Scalars['String']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<QuoteLineSortInput>>;
};


export type FullQueryQuoteSummaryArgs = {
  tenderRequestId: Scalars['String'];
};


export type FullQueryQuoteUploadsArgs = {
  organisationId?: InputMaybe<Scalars['String']>;
  quoteId: Scalars['String'];
};


export type FullQueryQuoteUploadsByTenderRequestIdArgs = {
  organisationId?: InputMaybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
};


export type FullQueryQuotesArgs = {
  filter?: InputMaybe<QuoteFilterInput>;
  organisationId?: InputMaybe<Scalars['String']>;
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
  organisationId: Scalars['String'];
  quoteId: Scalars['String'];
  tenderRequestId: Scalars['String'];
};


export type FullQueryTenderRequestLinesArgs = {
  filter?: InputMaybe<TenderRequestLineFilterInput>;
  organisationId: Scalars['String'];
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<TenderRequestLineSortInput>>;
};


export type FullQueryTenderRequestLinesByTenderRequestIdArgs = {
  organisationId: Scalars['String'];
  tenderRequestId: Scalars['String'];
};


export type FullQueryTenderRequestNoticesArgs = {
  organisationId?: InputMaybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
};


export type FullQueryTenderRequestUploadsArgs = {
  organisationId?: InputMaybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
};


export type FullQueryTenderRequestsArgs = {
  filter?: InputMaybe<TenderRequestFilterInput>;
  organisationId?: InputMaybe<Scalars['String']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<TenderRequestSortInput>>;
};


export type FullQueryUserAccountsArgs = {
  filter?: InputMaybe<UserAccountFilterInput>;
  organisationId?: InputMaybe<Scalars['String']>;
  page?: InputMaybe<PaginationInput>;
  sort?: InputMaybe<Array<UserAccountSortInput>>;
};

export type GroupMemberConnector = {
  __typename: 'GroupMemberConnector';
  nodes: Array<GroupMemberNode>;
  totalCount: Scalars['Int'];
};

export type GroupMemberFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  organisationGroupId?: InputMaybe<EqualFilterStringInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
  organisationName?: InputMaybe<StringFilterInput>;
};

export type GroupMemberNode = {
  __typename: 'GroupMemberNode';
  id: Scalars['String'];
  organisation?: Maybe<OrganisationNode>;
  organisationGroupId: Scalars['String'];
  organisationId: Scalars['String'];
};

export type GroupMemberResponse = GroupMemberConnector;

export enum GroupMemberSortFieldInput {
  Name = 'name'
}

export type GroupMemberSortInput = {
  desc?: InputMaybe<Scalars['Boolean']>;
  key: GroupMemberSortFieldInput;
};

export type IdResponse = {
  __typename: 'IdResponse';
  id: Scalars['String'];
};

export type InsertGroupMemberInput = {
  id: Scalars['String'];
  organisationGroupId: Scalars['String'];
  organisationId: Scalars['String'];
};

export type InsertGroupMemberResponse = GroupMemberNode;

export type InsertManufacturerInput = {
  code: Scalars['String'];
  country?: InputMaybe<Scalars['String']>;
  id: Scalars['String'];
  name: Scalars['String'];
};

export type InsertManufacturerResponse = ManufacturerNode;

export type InsertOrganisationGroupInput = {
  id: Scalars['String'];
  name: Scalars['String'];
  organisationId: Scalars['String'];
};

export type InsertOrganisationGroupResponse = OrganisationGroupNode;

export type InsertOrganisationInput = {
  code: Scalars['String'];
  contactName?: InputMaybe<Scalars['String']>;
  description?: InputMaybe<Scalars['String']>;
  email?: InputMaybe<Scalars['String']>;
  id: Scalars['String'];
  isCustomer: Scalars['Boolean'];
  isManufacturer: Scalars['Boolean'];
  isSupplier: Scalars['Boolean'];
  name: Scalars['String'];
  phone?: InputMaybe<Scalars['String']>;
  website?: InputMaybe<Scalars['String']>;
};

export type InsertOrganisationResponse = OrganisationNode;

export type InsertQuoteInput = {
  currency: Scalars['String'];
  freightType: Scalars['String'];
  id: Scalars['String'];
  organisationId: Scalars['String'];
  statusReason: Scalars['String'];
  supplierNotes?: InputMaybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
};

export type InsertQuoteResponse = QuoteNode;

export type InsertTenderRequestInput = {
  closingDatetime: Scalars['DateTime'];
  description: Scalars['String'];
  id: Scalars['String'];
  incoterm?: InputMaybe<Scalars['String']>;
  organisationId: Scalars['String'];
  publishedDatetime?: InputMaybe<Scalars['DateTime']>;
};

export type InsertTenderRequestLineInput = {
  comments: Scalars['String'];
  conditions: Scalars['String'];
  id: Scalars['String'];
  itemCode: Scalars['String'];
  itemId: Scalars['String'];
  itemName: Scalars['String'];
  msupplyLineNumber: Scalars['Int'];
  numberOfPacks: Scalars['Float'];
  preferredPackSize: Scalars['Float'];
  productSpecifications?: InputMaybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
  unitQuantity: Scalars['Float'];
  units: Scalars['String'];
  universalCode?: InputMaybe<Scalars['String']>;
};

export type InsertTenderRequestLineResponse = TenderRequestLineNode;

export type InsertTenderRequestResponse = TenderRequestNode;

export type InternalError = LogoutErrorInterface & RefreshTokenErrorInterface & {
  __typename: 'InternalError';
  description: Scalars['String'];
  fullError: Scalars['String'];
};

export type InvalidCredentials = AuthTokenErrorInterface & {
  __typename: 'InvalidCredentials';
  description: Scalars['String'];
};

export type InvalidToken = RefreshTokenErrorInterface & {
  __typename: 'InvalidToken';
  description: Scalars['String'];
};

export type InviteResponse = IdResponse;

export type InviteUserInput = {
  displayName: Scalars['String'];
  email: Scalars['String'];
  organisationId: Scalars['String'];
  permissions: Array<PermissionNode>;
  username: Scalars['String'];
};

export type InviteUserResponse = InviteUserResponseMessage;

export type InviteUserResponseMessage = {
  __typename: 'InviteUserResponseMessage';
  message: Scalars['String'];
};

export type LogConnector = {
  __typename: 'LogConnector';
  nodes: Array<LogNode>;
  totalCount: Scalars['Int'];
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
  datetime: Scalars['DateTime'];
  id: Scalars['String'];
  organisation?: Maybe<OrganisationNode>;
  organisationId?: Maybe<Scalars['String']>;
  recordId?: Maybe<Scalars['String']>;
  recordType: LogNodeType;
  user?: Maybe<UserAccountNode>;
};

export enum LogNodeType {
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
  desc?: InputMaybe<Scalars['Boolean']>;
  /** Sort query result by `key` */
  key: LogSortFieldInput;
};

export type Logout = {
  __typename: 'Logout';
  /** User id of the logged out user */
  userId: Scalars['String'];
};

export type LogoutError = {
  __typename: 'LogoutError';
  error: LogoutErrorInterface;
};

export type LogoutErrorInterface = {
  description: Scalars['String'];
};

export type LogoutResponse = Logout | LogoutError;

export type ManufacturerConnector = {
  __typename: 'ManufacturerConnector';
  nodes: Array<ManufacturerNode>;
  totalCount: Scalars['Int'];
};

export type ManufacturerFilterInput = {
  country?: InputMaybe<SimpleStringFilterInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  name?: InputMaybe<SimpleStringFilterInput>;
};

export type ManufacturerNode = {
  __typename: 'ManufacturerNode';
  auditLogs: Array<LogNode>;
  code: Scalars['String'];
  country?: Maybe<Scalars['String']>;
  id: Scalars['String'];
  name: Scalars['String'];
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
  desc?: InputMaybe<Scalars['Boolean']>;
  /** Sort query result by `key` */
  key: ManufacturerSortFieldInput;
};

export type ManufacturersResponse = ManufacturerConnector;

export type NoRefreshTokenProvided = RefreshTokenErrorInterface & {
  __typename: 'NoRefreshTokenProvided';
  description: Scalars['String'];
};

/** Generic Error Wrapper */
export type NodeError = {
  __typename: 'NodeError';
  error: NodeErrorInterface;
};

export type NodeErrorInterface = {
  description: Scalars['String'];
};

export type NotARefreshToken = RefreshTokenErrorInterface & {
  __typename: 'NotARefreshToken';
  description: Scalars['String'];
};

export type OrganisationConnector = {
  __typename: 'OrganisationConnector';
  nodes: Array<OrganisationNode>;
  totalCount: Scalars['Int'];
};

export type OrganisationFilterInput = {
  code?: InputMaybe<StringFilterInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  isCustomer?: InputMaybe<Scalars['Boolean']>;
  isManufacturer?: InputMaybe<Scalars['Boolean']>;
  isSupplier?: InputMaybe<Scalars['Boolean']>;
  modifiedBy?: InputMaybe<EqualFilterStringInput>;
  name?: InputMaybe<StringFilterInput>;
  search?: InputMaybe<Scalars['String']>;
};

export type OrganisationGroupConnector = {
  __typename: 'OrganisationGroupConnector';
  nodes: Array<OrganisationGroupNode>;
  totalCount: Scalars['Int'];
};

export type OrganisationGroupFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  name?: InputMaybe<SimpleStringFilterInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
};

export type OrganisationGroupNode = {
  __typename: 'OrganisationGroupNode';
  id: Scalars['String'];
  name: Scalars['String'];
  organisationId: Scalars['String'];
};

export type OrganisationGroupResponse = OrganisationGroupConnector;

export enum OrganisationGroupSortFieldInput {
  Name = 'name'
}

export type OrganisationGroupSortInput = {
  desc?: InputMaybe<Scalars['Boolean']>;
  key: OrganisationGroupSortFieldInput;
};

export type OrganisationNode = {
  __typename: 'OrganisationNode';
  auditLogs: Array<LogNode>;
  code: Scalars['String'];
  contactName?: Maybe<Scalars['String']>;
  country?: Maybe<Scalars['String']>;
  createdDatetime?: Maybe<Scalars['DateTime']>;
  description?: Maybe<Scalars['String']>;
  discoverable: Scalars['Boolean'];
  email?: Maybe<Scalars['String']>;
  entityType?: Maybe<Scalars['String']>;
  fullLegalName?: Maybe<Scalars['String']>;
  id: Scalars['String'];
  isCustomer: Scalars['Boolean'];
  isManufacturer: Scalars['Boolean'];
  isSupplier: Scalars['Boolean'];
  logoId?: Maybe<Scalars['String']>;
  modifiedBy: Scalars['String'];
  modifiedDatetime?: Maybe<Scalars['DateTime']>;
  name: Scalars['String'];
  operatingAddress?: Maybe<Scalars['String']>;
  phone?: Maybe<Scalars['String']>;
  postalAddress?: Maybe<Scalars['String']>;
  registrationJurisdiction?: Maybe<Scalars['String']>;
  registrationNumber?: Maybe<Scalars['String']>;
  website?: Maybe<Scalars['String']>;
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
  desc?: InputMaybe<Scalars['Boolean']>;
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
  first?: InputMaybe<Scalars['Int']>;
  /** First returned item is at the `offset` position in the full list */
  offset?: InputMaybe<Scalars['Int']>;
};

export type PasswordResetResponse = PasswordResetResponseMessage;

export type PasswordResetResponseMessage = {
  __typename: 'PasswordResetResponseMessage';
  message: Scalars['String'];
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
  totalCount: Scalars['Int'];
};

export type QuoteFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
  tenderRequestId?: InputMaybe<EqualFilterStringInput>;
};

export type QuoteLineConnector = {
  __typename: 'QuoteLineConnector';
  nodes: Array<QuoteLineNode>;
  totalCount: Scalars['Int'];
};

export type QuoteLineFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  quoteId?: InputMaybe<EqualFilterStringInput>;
};

export type QuoteLineNode = {
  __typename: 'QuoteLineNode';
  createdDatetime: Scalars['DateTime'];
  currency: Scalars['String'];
  deliveryLeadTime: Scalars['String'];
  expiryPeriod: Scalars['String'];
  id: Scalars['String'];
  itemCode: Scalars['String'];
  manufacturerId: Scalars['String'];
  methodOfDelivery: Scalars['String'];
  modifiedDatetime: Scalars['DateTime'];
  numberOfPacks: Scalars['Int'];
  packSize: Scalars['Float'];
  pricePerPack: Scalars['Float'];
  quoteId: Scalars['String'];
  supplierItemId?: Maybe<Scalars['String']>;
  supplierNotes: Scalars['String'];
  tenderRequestLineId: Scalars['String'];
};

export enum QuoteLineSortFieldInput {
  CreatedDatetime = 'createdDatetime'
}

export type QuoteLineSortInput = {
  /**
   * Sort query result is sorted descending or ascending (if not provided the default is
   * ascending)
   */
  desc?: InputMaybe<Scalars['Boolean']>;
  /** Sort query result by `key` */
  key: QuoteLineSortFieldInput;
};

export type QuoteLinesResponse = QuoteLineConnector;

export type QuoteNode = {
  __typename: 'QuoteNode';
  auditLogs: Array<LogNode>;
  createdDatetime: Scalars['DateTime'];
  currency: Scalars['String'];
  customerNotes?: Maybe<Scalars['String']>;
  finalisedDatetime?: Maybe<Scalars['DateTime']>;
  freightType: Scalars['String'];
  id: Scalars['String'];
  late: Scalars['Boolean'];
  lineCount: Scalars['Int'];
  modifiedDatetime: Scalars['DateTime'];
  organisation?: Maybe<OrganisationNode>;
  organisationId: Scalars['String'];
  status: QuoteNodeStatus;
  statusReason: Scalars['String'];
  supplierNotes?: Maybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
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
  desc?: InputMaybe<Scalars['Boolean']>;
  /** Sort query result by `key` */
  key: QuoteSortFieldInput;
};

export type QuoteSummariesResponse = QuoteSummaryConnector;

export type QuoteSummaryConnector = {
  __typename: 'QuoteSummaryConnector';
  nodes: Array<QuoteSummaryNode>;
  totalCount: Scalars['Int'];
};

export type QuoteSummaryNode = {
  __typename: 'QuoteSummaryNode';
  id: Scalars['String'];
  lineCount: Scalars['Int'];
  modifiedDatetime: Scalars['DateTime'];
  organisation?: Maybe<OrganisationNode>;
  organisationId: Scalars['String'];
  status: QuoteSummaryNodeStatus;
  statusReason: Scalars['String'];
  tenderRequestId: Scalars['String'];
};

export enum QuoteSummaryNodeStatus {
  Confirmed = 'CONFIRMED',
  Draft = 'DRAFT',
  Invited = 'INVITED',
  Withdrawn = 'WITHDRAWN'
}

export type QuoteUploadConnector = {
  __typename: 'QuoteUploadConnector';
  nodes: Array<QuoteUploadNode>;
  totalCount: Scalars['Int'];
};

export type QuoteUploadNode = {
  __typename: 'QuoteUploadNode';
  contentType: Scalars['String'];
  createdDatetime: Scalars['DateTime'];
  description: Scalars['String'];
  fileName: Scalars['String'];
  id: Scalars['String'];
  modifiedDatetime: Scalars['DateTime'];
  organisation?: Maybe<OrganisationNode>;
  organisationId?: Maybe<Scalars['String']>;
  quoteId: Scalars['String'];
};

export type QuoteUploadsResponse = QuoteUploadConnector;

export type QuotesResponse = QuoteConnector;

export type RecordNotFound = NodeErrorInterface & {
  __typename: 'RecordNotFound';
  description: Scalars['String'];
};

export type RefreshToken = {
  __typename: 'RefreshToken';
  /** New Bearer token */
  token: Scalars['String'];
};

export type RefreshTokenError = {
  __typename: 'RefreshTokenError';
  error: RefreshTokenErrorInterface;
};

export type RefreshTokenErrorInterface = {
  description: Scalars['String'];
};

export type RefreshTokenResponse = RefreshToken | RefreshTokenError;

export type SimpleStringFilterInput = {
  /** Search term must be an exact match (case sensitive) */
  equalTo?: InputMaybe<Scalars['String']>;
  /** Search term must be included in search candidate (case insensitive) */
  like?: InputMaybe<Scalars['String']>;
};

export type StringFilterInput = {
  endsWith?: InputMaybe<Scalars['String']>;
  equalAny?: InputMaybe<Array<Scalars['String']>>;
  /** Search term must be an exact match (case sensitive) */
  equalTo?: InputMaybe<Scalars['String']>;
  isNull?: InputMaybe<Scalars['Boolean']>;
  /** Search term must be included in search candidate (case insensitive) */
  like?: InputMaybe<Scalars['String']>;
  notEqualAll?: InputMaybe<Array<Scalars['String']>>;
  notEqualTo?: InputMaybe<Scalars['String']>;
  startsWith?: InputMaybe<Scalars['String']>;
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
  desc?: InputMaybe<Scalars['Boolean']>;
  key: TenderQuestionSortFieldInput;
};

export type TenderQuoteLineConnector = {
  __typename: 'TenderQuoteLineConnector';
  nodes: Array<TenderQuoteLineNode>;
  totalCount: Scalars['Int'];
};

export type TenderQuoteLineFilterInput = {
  isQuoted?: InputMaybe<Scalars['Boolean']>;
  itemName?: InputMaybe<StringFilterInput>;
};

export type TenderQuoteLineNode = {
  __typename: 'TenderQuoteLineNode';
  conditions: Scalars['String'];
  currency: Scalars['String'];
  id: Scalars['String'];
  itemName: Scalars['String'];
  pricePerPack: Scalars['Float'];
  productSpecifications: Scalars['String'];
  quoteLineId: Scalars['String'];
  quotedNumberOfPacks: Scalars['Int'];
  requestedNumberOfPacks: Scalars['Float'];
  supplierNotes: Scalars['String'];
  tenderRequestId: Scalars['String'];
  totalPrice: Scalars['Float'];
  unitQuantity: Scalars['Float'];
};

export type TenderQuoteLinesResponse = TenderQuoteLineConnector;

export type TenderRequestConnector = {
  __typename: 'TenderRequestConnector';
  nodes: Array<TenderRequestNode>;
  totalCount: Scalars['Int'];
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
  totalCount: Scalars['Int'];
};

export type TenderRequestLineFilterInput = {
  id?: InputMaybe<EqualFilterStringInput>;
  itemName?: InputMaybe<StringFilterInput>;
  tenderRequestId?: InputMaybe<EqualFilterStringInput>;
};

export type TenderRequestLineNode = {
  __typename: 'TenderRequestLineNode';
  comments: Scalars['String'];
  conditions: Scalars['String'];
  id: Scalars['String'];
  itemCode: Scalars['String'];
  itemId: Scalars['String'];
  itemName: Scalars['String'];
  numberOfPacks: Scalars['Float'];
  preferredPackSize: Scalars['Float'];
  productSpecifications?: Maybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
  unitQuantity: Scalars['Float'];
  universalCode?: Maybe<Scalars['String']>;
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
  desc?: InputMaybe<Scalars['Boolean']>;
  /** Sort query result by `key` */
  key: TenderRequestLineSortFieldInput;
};

export type TenderRequestLinesResponse = TenderRequestLineConnector;

export type TenderRequestNode = {
  __typename: 'TenderRequestNode';
  auditLogs: Array<LogNode>;
  closingDatetime: Scalars['DateTime'];
  createdDatetime: Scalars['DateTime'];
  description: Scalars['String'];
  id: Scalars['String'];
  incoterm?: Maybe<Scalars['String']>;
  lineCount: Scalars['Int'];
  modifiedDatetime: Scalars['DateTime'];
  organisation?: Maybe<OrganisationNode>;
  organisationId: Scalars['String'];
  publishedDatetime?: Maybe<Scalars['DateTime']>;
  reason?: Maybe<Scalars['String']>;
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
  totalCount: Scalars['Int'];
};

export type TenderRequestNoticeNode = {
  __typename: 'TenderRequestNoticeNode';
  body: Scalars['String'];
  createdBy: Scalars['String'];
  createdDatetime: Scalars['DateTime'];
  id: Scalars['String'];
  modifiedBy: Scalars['String'];
  modifiedDatetime: Scalars['DateTime'];
  originalId: Scalars['String'];
  tenderRequestId: Scalars['String'];
  title: Scalars['String'];
};

export type TenderRequestNoticesResponse = TenderRequestNoticeConnector;

export type TenderRequestQuestionConnector = {
  __typename: 'TenderRequestQuestionConnector';
  nodes: Array<TenderRequestQuestionNode>;
  totalCount: Scalars['Int'];
};

export type TenderRequestQuestionNode = {
  __typename: 'TenderRequestQuestionNode';
  answer?: Maybe<Scalars['String']>;
  answeredBy?: Maybe<Scalars['String']>;
  answeredDatetime?: Maybe<Scalars['DateTime']>;
  askingOrganisation: Scalars['String'];
  createdBy: Scalars['String'];
  createdDatetime: Scalars['DateTime'];
  id: Scalars['String'];
  modifiedBy: Scalars['String'];
  modifiedDatetime: Scalars['DateTime'];
  noticeId?: Maybe<Scalars['String']>;
  question: Scalars['String'];
  tenderRequestId: Scalars['String'];
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
  desc?: InputMaybe<Scalars['Boolean']>;
  /** Sort query result by `key` */
  key: TenderRequestSortFieldInput;
};

export type TenderRequestUploadConnector = {
  __typename: 'TenderRequestUploadConnector';
  nodes: Array<TenderRequestUploadNode>;
  totalCount: Scalars['Int'];
};

export type TenderRequestUploadNode = {
  __typename: 'TenderRequestUploadNode';
  contentType: Scalars['String'];
  createdDatetime: Scalars['DateTime'];
  description: Scalars['String'];
  fileName: Scalars['String'];
  id: Scalars['String'];
  modifiedDatetime: Scalars['DateTime'];
  tenderRequestId: Scalars['String'];
};

export type TenderRequestUploadsResponse = TenderRequestUploadConnector;

export type TenderRequestsResponse = TenderRequestConnector;

export type TokenExpired = RefreshTokenErrorInterface & {
  __typename: 'TokenExpired';
  description: Scalars['String'];
};

export type UpdateManufacturerInput = {
  code?: InputMaybe<Scalars['String']>;
  country?: InputMaybe<Scalars['String']>;
  id: Scalars['String'];
  name?: InputMaybe<Scalars['String']>;
};

export type UpdateManufacturerResponse = ManufacturerNode;

export type UpdateOrganisationGroupInput = {
  id: Scalars['String'];
  name?: InputMaybe<Scalars['String']>;
  organisationId?: InputMaybe<Scalars['String']>;
};

export type UpdateOrganisationGroupResponse = OrganisationGroupNode;

export type UpdateOrganisationInput = {
  code?: InputMaybe<Scalars['String']>;
  contactName?: InputMaybe<Scalars['String']>;
  country?: InputMaybe<Scalars['String']>;
  description?: InputMaybe<Scalars['String']>;
  discoverable?: InputMaybe<Scalars['Boolean']>;
  email?: InputMaybe<Scalars['String']>;
  entityType?: InputMaybe<Scalars['String']>;
  fullLegalName?: InputMaybe<Scalars['String']>;
  id: Scalars['String'];
  isCustomer?: InputMaybe<Scalars['Boolean']>;
  isManufacturer?: InputMaybe<Scalars['Boolean']>;
  isSupplier?: InputMaybe<Scalars['Boolean']>;
  logoId?: InputMaybe<Scalars['String']>;
  name?: InputMaybe<Scalars['String']>;
  operatingAddress?: InputMaybe<Scalars['String']>;
  phone?: InputMaybe<Scalars['String']>;
  postalAddress?: InputMaybe<Scalars['String']>;
  registrationJurisdiction?: InputMaybe<Scalars['String']>;
  registrationNumber?: InputMaybe<Scalars['String']>;
  website?: InputMaybe<Scalars['String']>;
};

export type UpdateOrganisationResponse = OrganisationNode;

export type UpdateOwnUserAccountInput = {
  displayName?: InputMaybe<Scalars['String']>;
  email?: InputMaybe<Scalars['String']>;
  id: Scalars['String'];
  password?: InputMaybe<Scalars['String']>;
  username?: InputMaybe<Scalars['String']>;
};

export type UpdateQuoteInput = {
  id: Scalars['String'];
  status?: InputMaybe<QuoteNodeStatus>;
  statusReason?: InputMaybe<Scalars['String']>;
  supplierNotes?: InputMaybe<Scalars['String']>;
};

export type UpdateQuoteResponse = QuoteNode;

export type UpdateTenderRequestInput = {
  closingDatetime?: InputMaybe<Scalars['DateTime']>;
  description?: InputMaybe<Scalars['String']>;
  id: Scalars['String'];
  reason?: InputMaybe<Scalars['String']>;
  status?: InputMaybe<TenderRequestNodeStatus>;
};

export type UpdateTenderRequestResponse = TenderRequestNode;

export type UpdateUserAccountInput = {
  displayName?: InputMaybe<Scalars['String']>;
  email?: InputMaybe<Scalars['String']>;
  id: Scalars['String'];
  password?: InputMaybe<Scalars['String']>;
  permissions?: InputMaybe<Array<PermissionNode>>;
  username?: InputMaybe<Scalars['String']>;
};

export type UpdateUserAccountResponse = UserAccountNode;

export type UpsertAnswerInput = {
  answer: Scalars['String'];
  id: Scalars['String'];
  noticeId?: InputMaybe<Scalars['String']>;
  tenderRequestId: Scalars['String'];
};

export type UpsertAnswerResponse = NodeError | TenderRequestQuestionNode;

export type UpsertQuoteLineInput = {
  comments: Scalars['String'];
  currency: Scalars['String'];
  deliveryLeadTime: Scalars['String'];
  expiryPeriod: Scalars['String'];
  id: Scalars['String'];
  itemCode: Scalars['String'];
  itemName: Scalars['String'];
  manufacturerName: Scalars['String'];
  methodOfDelivery: Scalars['String'];
  numberOfPacks: Scalars['Int'];
  packSize: Scalars['Float'];
  pricePerPack: Scalars['Float'];
  quoteId: Scalars['String'];
  supplierItemId?: InputMaybe<Scalars['String']>;
  tenderRequestLineId: Scalars['String'];
};

export type UpsertQuoteLineResponse = QuoteLineNode;

export type UpsertTenderRequestNoticeInput = {
  body: Scalars['String'];
  id: Scalars['String'];
  originalId: Scalars['String'];
  tenderRequestId: Scalars['String'];
  title: Scalars['String'];
};

export type UpsertTenderRequestNoticeResponse = TenderRequestNoticeNode;

export type UserAccountConnector = {
  __typename: 'UserAccountConnector';
  nodes: Array<UserAccountNode>;
  totalCount: Scalars['Int'];
};

export type UserAccountFilterInput = {
  displayName?: InputMaybe<SimpleStringFilterInput>;
  id?: InputMaybe<EqualFilterStringInput>;
  organisationId?: InputMaybe<EqualFilterStringInput>;
  search?: InputMaybe<Scalars['String']>;
  username?: InputMaybe<SimpleStringFilterInput>;
};

export type UserAccountNode = {
  __typename: 'UserAccountNode';
  auditLogs: Array<LogNode>;
  displayName: Scalars['String'];
  email?: Maybe<Scalars['String']>;
  id: Scalars['String'];
  organisation?: Maybe<OrganisationNode>;
  organisationId?: Maybe<Scalars['String']>;
  permissions: Array<PermissionNode>;
  username: Scalars['String'];
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
  desc?: InputMaybe<Scalars['Boolean']>;
  /** Sort query result by `key` */
  key: UserAccountSortFieldInput;
};

export type UserAccountsResponse = UserAccountConnector;

export type UserResponse = UserAccountNode;
