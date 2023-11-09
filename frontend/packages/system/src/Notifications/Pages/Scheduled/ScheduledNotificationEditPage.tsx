import React, { useEffect, useState } from 'react';
import {
  useTranslation,
  useNotification,
  useParams,
  useBreadcrumbs,
  FnUtils,
} from '@notify-frontend/common';

import { ConfigStatus } from '@common/types';

import { ScheduledNotificationEditForm } from './ScheduledNotificationEditForm';
import { BaseNotificationEditPage } from '../Base/BaseNotificationEditPage';
import { ScheduledNotification } from '../../types';
import {
  buildScheduledNotificationInputs,
  defaultSchedulerNotification,
  parseScheduledNotificationConfig,
} from './parseConfig';
import { useUpdateNotificationConfig } from '../../api/hooks/useUpdateNotificationConfig';
import { useDuplicateNotificationConfig } from '../../api/hooks/useDuplicateNotificationConfig';
import {
  NotificationConfigRowFragment,
  useNotificationConfigs,
} from '../../api';

export const ScheduledNotificationEditPage = () => {
  const t = useTranslation('system');
  const { error } = useNotification();
  const parsingErrorSnack = error(t('error.parsing-notification-config'));
  const { setSuffix } = useBreadcrumbs();

  const { id } = useParams<{ id: string }>();
  const [draft, setDraft] = useState<ScheduledNotification>(
    defaultSchedulerNotification
  );

  // Get the notification config from the API
  const { data, isLoading } = useNotificationConfigs({
    filterBy: { id: { equalTo: id } },
  });

  useEffect(() => {
    const entity = data?.nodes[0];
    // Once we get the notification config from the API, parse it and load into the draft
    const parsedDraft = parseScheduledNotificationConfig(
      (entity as NotificationConfigRowFragment) ?? null,
      parsingErrorSnack
    );
    setDraft(parsedDraft ?? defaultSchedulerNotification);
    if (parsedDraft?.title) setSuffix(parsedDraft?.title);
  }, [data]);

  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateNotificationConfig();

  const { mutateAsync: duplicate } =
    useDuplicateNotificationConfig();

  const onSave = async (draft: ScheduledNotification) => {
    const inputs = buildScheduledNotificationInputs(draft);
    await update({ input: inputs.update });
  };

  const onDuplicate = async (draft: ScheduledNotification) => {
    draft.id=FnUtils.generateUUID();
    draft.title=draft.title + " Copy";
    // want to check if there is the same title 

    draft.status= ConfigStatus.Disabled;
    const inputs = buildScheduledNotificationInputs(draft);
    await duplicate({ input: inputs.duplicate });
  };

  const isInvalid =
    !draft.title ||
    // nothing selected
    // no recipients selected
    (!draft.recipientListIds.length &&
      !draft.recipientIds.length &&
      !draft.sqlRecipientListIds.length);

  return (
    <BaseNotificationEditPage
      isLoading={isLoading || updateIsLoading}
      isInvalid={isInvalid}
      allowParameterSets={true}
      onSave={onSave}
      onDuplicate={onDuplicate}
      draft={draft}
      setDraft={setDraft}
      CustomForm={ScheduledNotificationEditForm}
    />
  );
};
