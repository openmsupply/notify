import React, { FC } from 'react';
import { RouteBuilder, Navigate, useMatch } from '@notify-frontend/common';
import { AppRoute } from '@notify-frontend/config';

const RecipientsService = React.lazy(
  () => import('@notify-frontend/system/src/Recipients/Service/Service')
);

const fullRecipientsPath = RouteBuilder.create(AppRoute.Recipients)
  .addWildCard()
  .build();

export const RecipientsRouter: FC = () => {
  const goToRecipients = useMatch(fullRecipientsPath);

  if (goToRecipients) {
    return <RecipientsService />;
  }

  const notFoundRoute = RouteBuilder.create(AppRoute.PageNotFound).build();
  return <Navigate to={notFoundRoute} />;
};
