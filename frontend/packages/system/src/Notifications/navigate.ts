import { ConfigKind } from '@common/types';
import { RouteBuilder } from '@common/utils';
import { AppRoute } from 'packages/config/src';

export function configRoute(kind: ConfigKind, id: string): string {
  switch (kind) {
    case ConfigKind.ColdChain:
      return RouteBuilder.create(AppRoute.Notifications)
        .addPart(AppRoute.ColdChain)
        .addPart(id)
        .build();
    case ConfigKind.Scheduled:
      return RouteBuilder.create(AppRoute.Notifications)
        .addPart(AppRoute.Scheduled)
        .addPart(id)
        .build();
    default:
      const exhaustiveCheck: never = kind;
      console.error(`Unhandled kind: ${exhaustiveCheck}`);
      // The `never` type is used here to ensure that all possible enum values are handled.
      break;
  }
  return '';
}
