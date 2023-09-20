import React from 'react';
import { Routes, Route, ConfigKind } from '@notify-frontend/common';
import { AppRoute } from 'packages/config/src';
import { ListView } from '../ListView/ListView';
import { CCNotificationEditPage } from '../Pages/ColdChain/CCNotificationEditPage';
import { ScheduledNotificationEditPage } from '../Pages/Scheduled/ScheduledNotificationEditPage';

const NotificationsService = () => {
  return (
    <Routes>
      <Route path="/" element={<ListView kind={null} />} />
      <Route
        path={`${AppRoute.ColdChain}`}
        element={<ListView kind={ConfigKind.ColdChain} />}
      />
      <Route
        path={`${AppRoute.ColdChain}/:id`}
        element={<CCNotificationEditPage />}
      />
      <Route
        path={`${AppRoute.Scheduled}`}
        element={<ListView kind={ConfigKind.Scheduled} />}
      />
      <Route
        path={`${AppRoute.Scheduled}/:id`}
        element={<ScheduledNotificationEditPage />}
      />
    </Routes>
  );
};

export default NotificationsService;
