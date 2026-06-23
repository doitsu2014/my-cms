import { z } from 'zod';
import { UserRoleEnum } from '@/domains/user';

export const createUserSchema = z.object({
  email: z.string().email('Invalid email'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
  role: z.enum([UserRoleEnum.Administrator, UserRoleEnum.Writer]),
});
export type CreateUserFormData = z.infer<typeof createUserSchema>;

export const modifyUserSchema = z.object({
  email: z.string().email('Invalid email'),
  role: z.enum([UserRoleEnum.Administrator, UserRoleEnum.Writer]),
  banned: z.boolean().default(false),
});
export type ModifyUserFormData = z.infer<typeof modifyUserSchema>;
