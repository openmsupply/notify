import React, { FC } from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { UserAccountListView } from '../ListView';

export const UserAccountService: FC = () => {
  return (
    <Routes>
      <Route path="" element={<UserAccountListView />} />
    </Routes>
  );
};

export default UserAccountService;
