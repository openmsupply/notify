import * as Types from '@notify-frontend/common';

import { GraphQLClient } from 'graphql-request';
import * as Dom from 'graphql-request/dist/types.dom';
import gql from 'graphql-tag';
export type MyAccountRowFragment = { __typename: 'UserAccountNode', id: string, username: string, email?: string | null, displayName: string };

export type UpdateOwnUserAccountMutationVariables = Types.Exact<{
  input: Types.UpdateOwnUserAccountInput;
}>;


export type UpdateOwnUserAccountMutation = { __typename: 'FullMutation', updateOwnUserAccount: { __typename: 'UserAccountNode', id: string, username: string, email?: string | null, displayName: string } };

export const MyAccountRowFragmentDoc = gql`
    fragment MyAccountRow on UserAccountNode {
  __typename
  id
  username
  email
  displayName
}
    `;
export const UpdateOwnUserAccountDocument = gql`
    mutation updateOwnUserAccount($input: UpdateOwnUserAccountInput!) {
  updateOwnUserAccount(input: $input) {
    ... on UserAccountNode {
      ...MyAccountRow
    }
  }
}
    ${MyAccountRowFragmentDoc}`;

export type SdkFunctionWrapper = <T>(action: (requestHeaders?:Record<string, string>) => Promise<T>, operationName: string, operationType?: string) => Promise<T>;


const defaultWrapper: SdkFunctionWrapper = (action, _operationName, _operationType) => action();

export function getSdk(client: GraphQLClient, withWrapper: SdkFunctionWrapper = defaultWrapper) {
  return {
    updateOwnUserAccount(variables: UpdateOwnUserAccountMutationVariables, requestHeaders?: Dom.RequestInit["headers"]): Promise<UpdateOwnUserAccountMutation> {
      return withWrapper((wrappedRequestHeaders) => client.request<UpdateOwnUserAccountMutation>(UpdateOwnUserAccountDocument, variables, {...requestHeaders, ...wrappedRequestHeaders}), 'updateOwnUserAccount', 'mutation');
    }
  };
}
export type Sdk = ReturnType<typeof getSdk>;