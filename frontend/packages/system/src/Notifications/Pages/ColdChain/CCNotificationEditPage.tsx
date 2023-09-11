import React, { FC, useEffect, useState } from 'react';
import {
  ModalMode,
  FnUtils,
  ConfigKind,
  useTranslation,
  useNotification,
  useParams,
} from '@notify-frontend/common';
import { CCNotificationEditForm } from './CCNotificationEditForm';
import { BaseNotificationEditPage } from '../Base/BaseNotificationEditPage';
import { CCNotification } from '../../types';
import { useCreateNotificationConfig } from '../../api/hooks/useCreateNotificationConfig';
import {
  buildColdChainNotificationInputs,
  parseColdChainNotificationConfig,
} from './parseConfig';
import { useUpdateNotificationConfig } from '../../api/hooks/useUpdateNotificationConfig';
import {
  NotificationConfigRowFragment,
  useNotificationConfigs,
} from '../../api';

interface CCNotificationEditPageProps {
  mode: ModalMode | null;
}

function useNewCCNotification() {
  const newList = {
    __typename: 'NotificationConfigNode',
    id: FnUtils.generateUUID(),
    title: '',
    kind: ConfigKind.ColdChain,
    configurationData: '{}',
  } as NotificationConfigRowFragment;
  return { data: { nodes: [newList] }, isError: false, isLoading: false };
}

const createCCNotification = (
  seed: CCNotification | null | undefined
): CCNotification => ({
  id: seed?.id ?? FnUtils.generateUUID(),
  title: seed?.title ?? '',
  kind: seed?.kind ?? ConfigKind.ColdChain,
  highTemp: seed?.highTemp ?? false,
  lowTemp: seed?.lowTemp ?? false,
  confirmOk: seed?.confirmOk ?? false,
  remind: seed?.remind ?? false,
  reminderInterval: seed?.reminderInterval ?? 5,
  reminderUnits: seed?.reminderUnits ?? 'minutes',
  locationIds: seed?.locationIds ?? [],
  recipientIds: seed?.recipientIds ?? [],
  recipientListIds: seed?.recipientListIds ?? [],
});

export const CCNotificationEditPage: FC<CCNotificationEditPageProps> = ({
  mode,
}) => {
  const t = useTranslation('system');
  const { error } = useNotification();
  const parsingErrorSnack = error(t('error.parsing-notification-config'));

  const { id } = useParams<{ id: string }>();

  // Get the notification config from the API
  const { data, isLoading } =
    id === 'new'
      ? useNewCCNotification()
      : useNotificationConfigs({
          filterBy: { id: { equalTo: id } },
        });
  const entity = data?.nodes[0];

  const [draft, setDraft] = useState<CCNotification>(() =>
    createCCNotification(null)
  );

  useEffect(() => {
    const parsedDraft = parseColdChainNotificationConfig(
      entity ?? null,
      parsingErrorSnack
    );
    setDraft(createCCNotification(parsedDraft));
  }, []);

  const { mutateAsync: create, isLoading: createIsLoading } =
    useCreateNotificationConfig();

  const { mutateAsync: update, isLoading: updateIsLoading } =
    useUpdateNotificationConfig();

  const onSave = async (draft: CCNotification) => {
    const inputs = buildColdChainNotificationInputs(draft);

    if (mode === ModalMode.Create) {
      await create({ input: inputs.create });
    } else {
      await update({ input: inputs.update });
    }
  };

  const isInvalid =
    !draft.title ||
    // nothing selected
    (!draft.confirmOk && !draft.highTemp && !draft.lowTemp && draft.remind) ||
    // no locations selected
    !draft.locationIds.length ||
    // no recipients selected
    (!draft.recipientListIds.length && !draft.recipientIds.length);

  return (
    <BaseNotificationEditPage
      kind={ConfigKind.ColdChain}
      isLoading={isLoading || createIsLoading || updateIsLoading}
      isInvalid={isInvalid}
      onSave={onSave}
      draft={draft}
      setDraft={setDraft}
      CustomForm={CCNotificationEditForm}
    />
  );
};
