import React from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { AppRoute } from 'packages/config/src';
import { DetailView, AllLists } from '../RecipientLists';
import { DetailView as DetailSqlView, AllSqlLists } from '../SqlRecipientLists';
import { ListView } from '../Recipients';

const RecipientsService = () => {
  return (
    <Routes>
      <Route path="/" element={<ListView />} />
      <Route path={AppRoute.RecipientLists} element={<AllLists />} />
      <Route path={AppRoute.SqlRecipientLists} element={<AllSqlLists />} />
      <Route
        path={`${AppRoute.RecipientLists}/:listId`}
        element={<DetailView />}
      />
      <Route
        path={`${AppRoute.SqlRecipientLists}/:listId`}
        element={<DetailSqlView />}
      />
    </Routes>
  );
};

export default RecipientsService;
