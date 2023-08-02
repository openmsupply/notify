import React from 'react';
import { AppRoute } from '@notify-frontend/config';
import { render } from '@testing-library/react';
import {
  RouteBuilder,
  TestingProvider,
  TestingRouter,
} from '../../../../utils';
import { Route } from 'react-router';
import { Breadcrumbs } from './Breadcrumbs';

describe('Breadcrumbs', () => {
  it('does not render the top level part of the current URL', () => {
    const { queryByText } = render(
      <TestingProvider>
        <TestingRouter
          initialEntries={[RouteBuilder.create(AppRoute.UserAccounts).build()]}
        >
          <Route path="*" element={<Breadcrumbs />}></Route>
        </TestingRouter>
      </TestingProvider>
    );

    expect(queryByText(/users/i)).not.toBeInTheDocument();
  });
  it('Renders the names of all the routes from the URL, excluding the first', () => {
    const { getByText } = render(
      <TestingProvider>
        <TestingRouter
          initialEntries={[
            RouteBuilder.create(AppRoute.UserAccounts)
              .addPart(AppRoute.Admin)
              .addPart(AppRoute.MyAccount)
              .build(),
          ]}
        >
          <Route path="*" element={<Breadcrumbs />}></Route>
        </TestingRouter>
      </TestingProvider>
    );

    expect(getByText(/admin/i));
    expect(getByText(/my-account/i));
  });
});
