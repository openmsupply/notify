import React from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { ListView } from '../ListView/ListView';

const NotificationsService = () => {
  return (
    <Routes>
      <Route path="/" element={<ListView />} />
    </Routes>
  );
};

export default NotificationsService;
