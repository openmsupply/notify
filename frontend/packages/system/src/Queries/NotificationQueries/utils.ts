export function stringifyObjectKey(value: unknown) {
  // If we have a string or number, return it
  if (typeof value === 'string' || typeof value === 'number') {
    return value;
  }
  // If we have an object, stringify it
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }

  // If we have a boolean, return true or false
  if (typeof value === 'boolean') {
    return value ? 'true' : 'false';
  }

  // Otherwise, return unknown
  return 'unknown return type';
}
