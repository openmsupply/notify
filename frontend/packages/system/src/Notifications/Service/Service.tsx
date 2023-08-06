import React, { FC } from 'react';
import { Routes, Route } from '@notify-frontend/common';
import { AppRoute } from 'packages/config/src';
import { GroupDetails, GroupList } from '../Groups';

const NotificationsService: FC = () => {
  return (
    <Routes>
      <Route path={AppRoute.NotificationGroups} element={<GroupList />} />
      <Route
        path={`${AppRoute.NotificationGroups}/:groupId`}
        element={<GroupDetails />}
      />
    </Routes>
  );
};

export default NotificationsService;
