import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type LogRowFragment = { __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null };

export type LogsQueryVariables = Types.Exact<{
  filter?: Types.InputMaybe<Types.LogFilterInput>;
  page?: Types.InputMaybe<Types.PaginationInput>;
  sort?: Types.InputMaybe<Array<Types.LogSortInput> | Types.LogSortInput>;
}>;


export type LogsQuery = { __typename: 'FullQuery', logs: { __typename: 'LogConnector', totalCount: number, nodes: Array<{ __typename: 'LogNode', datetime: string, id: string, recordId?: string | null, recordType: Types.LogNodeType, user?: { __typename: 'UserAccountNode', username: string } | null }> } };

export const LogRowFragmentDoc = gql`
    fragment LogRow on LogNode {
  datetime
  id
  recordId
  recordType
  user {
    username
  }
}
    `;
export const LogsDocument = gql`
    query logs($filter: LogFilterInput, $page: PaginationInput, $sort: [LogSortInput!]) {
  logs(filter: $filter, page: $page, sort: $sort) {
    ... on LogConnector {
      nodes {
        ...LogRow
      }
      totalCount
    }
  }
}
    ${LogRowFragmentDoc}`;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    logs(variables?: LogsQueryVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<LogsQuery> {
      return withWrapper((wrappedRequestHeaders) => client.request<LogsQuery>(LogsDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'logs', 'query');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;