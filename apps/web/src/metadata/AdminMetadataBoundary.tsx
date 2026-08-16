import { useLocation } from 'react-router-dom';
import { useEffect } from 'react';
import { applyAdminMetadata } from './admin-metadata';

export default function AdminMetadataBoundary() {
  const location = useLocation();
  useEffect(() => {
    applyAdminMetadata(`${location.pathname}${location.search}`);
  }, [location.pathname, location.search]);
  return null;
}
