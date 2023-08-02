import React, { FC, useEffect } from 'react';

import {
  AppFooterPortal,
  Box,
  SnackbarProvider,
  AppFooter,
  Routes,
  Route,
  RouteBuilder,
  useLocation,
  useHostContext,
  useGetPageTitle,
} from '@notify-frontend/common';
import { AppDrawer, AppBar, Footer, NotFound } from './components';
import { CommandK } from './CommandK';
import { AppRoute } from '@notify-frontend/config';
import { Settings } from './Admin/Settings';
import { UserAccountRouter, MyAccountRouter } from './routers';
import { RequireAuthentication } from './components/Navigation/RequireAuthentication';
import { QueryErrorHandler } from './QueryErrorHandler';

export const Site: FC = () => {
  const location = useLocation();
  const getPageTitle = useGetPageTitle();
  const { setPageTitle } = useHostContext();

  useEffect(() => {
    setPageTitle(getPageTitle(location.pathname));
  }, [location]);

  return (
    <RequireAuthentication>
      <CommandK>
        <SnackbarProvider maxSnack={3}>
          <AppDrawer />
          <Box flex={1} display="flex" flexDirection="column" overflow="hidden">
            <AppBar />
            {/* <NotifyOnLogin /> */}
            <Box display="flex" flex={1} overflow="auto">
              <Routes>
                <Route
                  path={RouteBuilder.create(AppRoute.UserAccounts)
                    .addWildCard()
                    .build()}
                  element={<UserAccountRouter />}
                />
                <Route
                  path={RouteBuilder.create(AppRoute.Admin)
                    .addWildCard()
                    .build()}
                  element={<Settings />}
                />
                <Route
                  path={RouteBuilder.create(AppRoute.MyAccount)
                    .addWildCard()
                    .build()}
                  element={<MyAccountRouter />}
                />

                <Route path="*" element={<NotFound />} />
              </Routes>
            </Box>
            <AppFooter />
            <AppFooterPortal SessionDetails={<Footer />} />
          </Box>
          <QueryErrorHandler />
        </SnackbarProvider>
      </CommandK>
    </RequireAuthentication>
  );
};
