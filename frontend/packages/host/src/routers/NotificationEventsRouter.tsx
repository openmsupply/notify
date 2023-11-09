import React, { FC } from 'react';
import { RouteBuilder, Navigate, useMatch } from '@notify-frontend/common';
import { AppRoute } from '@notify-frontend/config';

const NotificationEventService = React.lazy(
  () => import('@notify-frontend/system/src/NotificationEvents/Service/Service')
);

const fullNotificationsPath = RouteBuilder.create(AppRoute.NotificationEvents)
  .addWildCard()
  .build();

export const NotificationEventsRouter: FC = () => {
  const goToNotifications = useMatch(fullNotificationsPath);

  if (goToNotifications) {
    return <NotificationEventService />;
  }

  const notFoundRoute = RouteBuilder.create(AppRoute.PageNotFound).build();
  return <Navigate to={notFoundRoute} />;
};
