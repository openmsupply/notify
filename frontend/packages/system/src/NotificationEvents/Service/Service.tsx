import React from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { ListView } from '../ListView/ListView';

const NotificationEventService = () => {
  return (
    <Routes>
      <Route path="/" element={<ListView kind={null} />} />
    </Routes>
  );
};

export default NotificationEventService;
