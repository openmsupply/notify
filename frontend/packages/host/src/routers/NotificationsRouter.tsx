import React, { FC } from 'react';
import { RouteBuilder, Navigate, useMatch } from '@notify-frontend/common';
import { AppRoute } from '@notify-frontend/config';

const NotificationsService = React.lazy(
  () => import('@notify-frontend/system/src/Notifications/Service/Service')
);

const fullNotificationsPath = RouteBuilder.create(AppRoute.Notifications)
  .addWildCard()
  .build();

export const NotificationsRouter: FC = () => {
  const gotoNotifications = useMatch(fullNotificationsPath);

  if (gotoNotifications) {
    return <NotificationsService />;
  }

  const notFoundRoute = RouteBuilder.create(AppRoute.PageNotFound).build();
  return <Navigate to={notFoundRoute} />;
};
