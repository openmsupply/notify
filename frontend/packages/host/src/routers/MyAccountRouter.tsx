import React from 'react';
import {
  RouteBuilder,
  Navigate,
  useMatch,
} from '@notify-frontend/common';
import { AppRoute } from '@notify-frontend/config';

const MyAccountService = React.lazy(
  () =>
    import('@notify-frontend/system/src/MyAccount/Service/Service')
);

const myAccountPath = RouteBuilder.create(AppRoute.MyAccount)
  .addWildCard()
  .build();

export const MyAccountRouter = () => {
  const gotoMyAccount = useMatch(myAccountPath);
  if (gotoMyAccount) {
    return <MyAccountService />;
  }

  const notFoundRoute = RouteBuilder.create(AppRoute.PageNotFound).build();
  return <Navigate to={notFoundRoute} />;
};
