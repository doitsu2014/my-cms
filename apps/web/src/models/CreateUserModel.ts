import type { UserRole } from '@/domains/user';

export interface CreateUserModel {
  email: string;
  password: string;
  role: UserRole;
}
