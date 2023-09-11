import React from 'react';
import { Routes, Route, ModalMode } from '@notify-frontend/common';
import { AppRoute } from 'packages/config/src';
import { ListView } from '../ListView/ListView';
import { CCNotificationEditPage } from '../Pages/ColdChain/CCNotificationEditPage';

const NotificationsService = () => {
  return (
    <Routes>
      <Route path="/" element={<ListView />} />
      <Route
        path={`${AppRoute.ColdChain}`}
        element={<CCNotificationEditPage mode={ModalMode.Create} />}
      />
      <Route
        path={`${AppRoute.ColdChain}/:id`}
        element={<CCNotificationEditPage mode={ModalMode.Update} />}
      />
    </Routes>
  );
};

export default NotificationsService;
