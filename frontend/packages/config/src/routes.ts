export enum AppRoute {
  Home = '',
  Login = 'login',
  PasswordReset = 'password-reset',
  ForgotPassword = 'forgot-password',

  UserAccounts = 'users',

  Notifications = 'notifications',
  NotificationEvents = 'notification-events',
  ColdChain = 'cold-chain',
  Scheduled = 'scheduled',

  Recipients = 'recipients',
  RecipientLists = 'lists',
  SqlRecipientLists = 'sql',

  Queries = 'queries',

  Admin = 'admin',
  Logout = 'logout',

  PageNotFound = 'page-not-found',

  VerifyAccount = 'verify-account',
}

export enum ExternalURL {
  PublicDocs = 'https://docs.msupply.foundation/notify',
}
