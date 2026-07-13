import type { UserRole } from '@/domains/user';

export interface ModifyUserModel {
  email?: string;
  role?: UserRole;
  banned?: boolean;
}
