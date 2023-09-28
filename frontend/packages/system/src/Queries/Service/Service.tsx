import React from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { DetailEdit, ListView } from '../NotificationQueries';

const NotificationQueryService = () => {
  return (
    <Routes>
      <Route path="/" element={<ListView />} />
      <Route path={`/:id`} element={<DetailEdit />} />
    </Routes>
  );
};

export default NotificationQueryService;
