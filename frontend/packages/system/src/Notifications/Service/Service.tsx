import React from 'react';
import { Routes, Route, ConfigKind } from '@notify-frontend/common';
import { AppRoute } from 'packages/config/src';
import { ListView } from '../ListView/ListView';
import { CCNotificationEditPage } from '../Pages/ColdChain/CCNotificationEditPage';

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
    </Routes>
  );
};

export default NotificationsService;
