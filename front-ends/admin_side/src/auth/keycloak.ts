import Keycloak from 'keycloak-js';
import { config } from '../config/runtime-config';

// Keycloak configuration from runtime config
const keycloakConfig = {
  url: config().keycloakUrl,
  realm: config().keycloakRealm,
  clientId: config().keycloakClientId,
};

// Create Keycloak instance
const keycloak = new Keycloak(keycloakConfig);

export default keycloak;
