import { renderOneOff } from '@openmsupply/tera-web';

const isTemplateError = (err: string) => {
  return err.startsWith('Template');
};
const isParamsError = (err: string) => {
  return err.startsWith('Parameter');
};

export const validateTemplate = (template: string) => {
  try {
    renderOneOff(template, JSON.stringify({}));
  } catch (e) {
    if (typeof e === 'string') {
      if (isTemplateError(e)) {
        throw e;
      }
      if (isParamsError(e)) {
        // Missing params are not an error when validating the template
        return;
      }
    }
    throw 'Unknown error :' + e;
  }
};
