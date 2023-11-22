export type EventContext = {
  [key: string]: unknown;
};

export const useParsedEventContext = (
  context: string | null | undefined
): EventContext => {
  if (!context) return {};
  try {
    return JSON.parse(context);
  } catch (e) {
    return {};
  }
};
