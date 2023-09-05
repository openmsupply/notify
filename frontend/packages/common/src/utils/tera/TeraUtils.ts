const parameterExtractorRE = /{{([^}]+)}}/g;

export const TeraUtils = {
  extractParams: function (templateText: string) {
    // Extracts parameters names from a template string
    // Example: extractParams('Hello {{name}}!') => ['name']
    return [...templateText.matchAll(parameterExtractorRE)].map(match =>
      match[1]?.trim()
    );
  },
};
