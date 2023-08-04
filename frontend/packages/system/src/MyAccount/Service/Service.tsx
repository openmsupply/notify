import React from 'react';
import { Routes, Route } from '@notify-frontend/common';
import MyAccountPage from '../MyAccountPage';

export const MyAccountService = () => {
  return (
    <Routes>
      <Route path="" element={<MyAccountPage />} />
    </Routes>
  );
};

export default MyAccountService;
