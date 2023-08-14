import { AppRoute } from '@notify-frontend/config';
import { RouteBuilder } from './RouteBuilder';

describe('Formatters', () => {
  it('builds a route with an appended wildcard', () => {
    expect(
      RouteBuilder.create(AppRoute.UserAccounts)
        .addPart(AppRoute.Admin)
        .addWildCard()
        .build()
    ).toBe('/users/admin/*');
  });

  it('builds a route', () => {
    expect(
      RouteBuilder.create(AppRoute.UserAccounts).addPart(AppRoute.Admin).build()
    ).toBe('/users/admin');
  });

  it('can be used to create multiple routes from the same builder', () => {
    expect(RouteBuilder.create(AppRoute.Admin).build()).toBe('/admin');
    expect(RouteBuilder.create(AppRoute.UserAccounts).build()).toBe('/users');
  });
});
