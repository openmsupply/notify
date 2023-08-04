import React from 'react';
import {
  LocaleKey,
  matchPath,
  RouteBuilder,
  SettingsIcon,
  SuppliersIcon,
  Tooltip,
  useLocation,
  UserIcon,
  UsersIcon,
  useTranslation,
} from '@notify-frontend/common';
import { AppRoute } from '@notify-frontend/config';

type Section = {
  icon?: JSX.Element;
  titleKey: LocaleKey;
};

const getIcon = (section?: AppRoute) => {
  switch (section) {
    case AppRoute.UserAccounts:
      return <UsersIcon color="primary" fontSize="small" />;
    case AppRoute.Admin:
      return <SettingsIcon color="primary" fontSize="small" />;
    case AppRoute.Notifications:
      return <SuppliersIcon color="primary" fontSize="small" />;
    case AppRoute.MyAccount:
      return <UserIcon color="primary" fontSize="small" />;
    default:
      return undefined;
  }
};

const getSection = (): Section | undefined => {
  const routes = [
    AppRoute.UserAccounts,
    AppRoute.Admin,
    AppRoute.MyAccount,
    AppRoute.Notifications,
  ];
  const location = useLocation();

  for (let i = 0; i < routes.length; i++) {
    const route = routes[i];
    const match = matchPath(
      RouteBuilder.create(route ?? '')
        .addWildCard()
        .build(),
      location.pathname
    );
    if (!!match)
      return {
        icon: getIcon(route),
        titleKey: route as LocaleKey,
      };
  }
  return undefined;
};

export const SectionIcon: React.FC = () => {
  const t = useTranslation(['host']);
  const section = getSection();

  return section?.icon ? (
    <Tooltip title={t(section?.titleKey)}>
      <div>{section.icon}</div>
    </Tooltip>
  ) : null;
};
