declare const API_HOST: string;
declare const APP_BUILD_VERSION: string;

// For production, API is on the same domain/ip and port as web app, available through sub-route
// i.e. web app is on https://my.healthsupplyhub.com/, then graphql will be available https://my.openmsupply.com/graphql

// For development, API server and front end are launched seperately on different ports and possible different IPs
// by default we assume development API server is launched on the same domain/ip and on port 8001 (Default). We can overwrite this
// with API_HOST which is available through webpack.DefinePlugin (i.e. webpack server --env API_HOST='localhost:9000')

const isProductionBuild = process.env['NODE_ENV'] === 'production';
const { port, hostname, protocol } = window.location;

const defaultDevelopmentApiHost = `${protocol}//${hostname}:8001`;
const productionApiHost = `${protocol}//${hostname}:${port}`;

const developmentApiHost =
  (typeof API_HOST !== 'undefined' && API_HOST) || defaultDevelopmentApiHost;

const apiHost = isProductionBuild ? productionApiHost : developmentApiHost;

const version =
  typeof APP_BUILD_VERSION !== 'undefined' && APP_BUILD_VERSION
    ? APP_BUILD_VERSION
    : '0.0.0';

export const Environment = {
  API_HOST: apiHost,
  BUILD_VERSION: version,
};

export default Environment;
