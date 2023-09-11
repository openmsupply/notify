import { ConfigKind } from '@common/types';
import { RouteBuilder } from '@common/utils';
import { AppRoute } from 'packages/config/src';

export function createConfigPath(kind: ConfigKind, id: string): string {
  if (kind === ConfigKind.ColdChain) {
    return RouteBuilder.create(AppRoute.Notifications)
      .addPart(AppRoute.ColdChain)
      .addPart(id)
      .build();
  } else {
    throw new Error(`UNKNOWN CONFIG KIND: ${kind}`);
  }
  return '';
}
