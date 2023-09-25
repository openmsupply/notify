import React, { FC } from 'react';
import { RouteBuilder, Navigate, useMatch } from '@notify-frontend/common';
import { AppRoute } from '@notify-frontend/config';

const QueriesService = React.lazy(
  () => import('@notify-frontend/system/src/Queries/Service/Service')
);

const fullQueriesPath = RouteBuilder.create(AppRoute.Queries)
  .addWildCard()
  .build();

export const QueriesRouter: FC = () => {
  const goToQueries = useMatch(fullQueriesPath);

  if (goToQueries) {
    return <QueriesService />;
  }

  const notFoundRoute = RouteBuilder.create(AppRoute.PageNotFound).build();
  return <Navigate to={notFoundRoute} />;
};
