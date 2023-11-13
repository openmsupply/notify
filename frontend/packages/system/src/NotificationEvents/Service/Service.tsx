import React from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { ListView } from '../ListView/ListView';
import { DetailView } from '../DetailView/DetailView';

const NotificationEventService = () => {
  return (
    <Routes>
      <Route path="/" element={<ListView kind={null} />} />
      <Route path={`/:id`} element={<DetailView />} />
    </Routes>
  );
};

export default NotificationEventService;
