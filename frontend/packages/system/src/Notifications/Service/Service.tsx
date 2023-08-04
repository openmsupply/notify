import React, { FC } from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { AppRoute } from 'packages/config/src';

const NotificationsService: FC = () => {
  return (
    <Routes>
      <Route path={AppRoute.NotificationGroups} element={<>Hello</>} />
    </Routes>
  );
};

export default NotificationsService;
