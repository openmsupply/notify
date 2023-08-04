import React, { FC } from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { AppRoute } from 'packages/config/src';
import { GroupList } from '../Groups';

const NotificationsService: FC = () => {
  return (
    <Routes>
      <Route path={AppRoute.NotificationGroups} element={<GroupList />} />
    </Routes>
  );
};

export default NotificationsService;
