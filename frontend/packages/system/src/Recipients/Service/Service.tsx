import React from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { AppRoute } from 'packages/config/src';
import { DetailView, AllLists } from '../RecipientLists';
import { ListView } from '../Recipients';

const RecipientsService = () => {
  return (
    <Routes>
      <Route path="/" element={<ListView />} />
      <Route path={AppRoute.RecipientLists} element={<AllLists />} />
      <Route
        path={`${AppRoute.RecipientLists}/:listId`}
        element={<DetailView />}
      />
    </Routes>
  );
};

export default RecipientsService;
