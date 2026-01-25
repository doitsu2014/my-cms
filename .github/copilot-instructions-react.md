# React Frontend Coding Guidelines for My-CMS

This document provides detailed guidelines for writing React and TypeScript code in the My-CMS frontend, with a focus on React best practices, DaisyUI component usage, and form handling.

## Table of Contents

1. [Component Architecture](#component-architecture)
2. [DaisyUI Component Usage](#daisyui-component-usage)
3. [Form Handling](#form-handling)
4. [State Management](#state-management)
5. [Data Fetching](#data-fetching)
6. [Routing](#routing)
7. [Authentication](#authentication)
8. [Code Examples](#code-examples)

## Component Architecture

### Component Structure

Follow a consistent structure for all React components:

```tsx
// 1. Imports
import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { SomeIcon } from 'lucide-react';

// 2. Types/Interfaces
interface ComponentProps {
  id?: string;
  onSubmit?: (data: FormData) => void;
}

// 3. Component Definition
export default function ComponentName({ id, onSubmit }: ComponentProps) {
  // 4. Hooks
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  
  // 5. Effects
  useEffect(() => {
    // Effect logic
  }, [dependencies]);
  
  // 6. Event Handlers
  const handleClick = () => {
    // Handler logic
  };
  
  // 7. Render
  return (
    <div>
      {/* JSX */}
    </div>
  );
}
```

### Component Organization

**Page Components** (in `src/app/*/page.tsx`):
- Responsible for data fetching
- Handle routing and navigation
- Pass data to presentational components

**Presentational Components** (in `src/components/`):
- Focus on UI rendering
- Receive data via props
- Minimal state management
- Reusable across pages

**Form Components**:
- Use React Hook Form for form state
- Integrate Zod for validation
- Handle submission and errors

### Naming Conventions

```tsx
// Components: PascalCase
export default function CategoryForm() {}
export function MediaGridItem() {}

// Functions: camelCase
function handleSubmit() {}
const fetchCategories = async () => {};

// Constants: SCREAMING_SNAKE_CASE
const MAX_FILE_SIZE = 5 * 1024 * 1024;
const AVAILABLE_LANGUAGES = ['en', 'vi'];

// Types/Interfaces: PascalCase
interface CategoryFormData {}
type PostStatus = 'draft' | 'published';
```

## DaisyUI Component Usage

### Overview

DaisyUI is configured through Tailwind CSS 4 using the `@plugin` directive in CSS files.

```css
/* App.css */
@import "tailwindcss";
@plugin "daisyui";
```

### Common DaisyUI Components

#### 1. Buttons

```tsx
// Primary button
<button className="btn btn-primary">
  <Save className="w-4 h-4" />
  Save
</button>

// Secondary button
<button className="btn btn-secondary">
  Cancel
</button>

// Outline button
<button className="btn btn-outline">
  Learn More
</button>

// Icon button
<button className="btn btn-circle btn-ghost">
  <X className="w-5 h-5" />
</button>

// Loading state
<button className="btn btn-primary" disabled={isLoading}>
  {isLoading && <span className="loading loading-spinner"></span>}
  {isLoading ? 'Saving...' : 'Save'}
</button>
```

#### 2. Cards

```tsx
// Basic card
<div className="card bg-base-100 shadow-xl">
  <div className="card-body">
    <h2 className="card-title">Card Title</h2>
    <p>Card content goes here</p>
    <div className="card-actions justify-end">
      <button className="btn btn-primary">Action</button>
    </div>
  </div>
</div>

// Card with border accent (common pattern in this project)
<div className="card bg-base-100 shadow-lg border-t-4 border-t-primary hover:shadow-xl transition-shadow duration-300">
  <div className="card-body">
    <div className="flex items-center justify-between">
      <h2 className="card-title text-lg">
        <FolderOpen className="w-5 h-5" />
        Section Title
      </h2>
      <span className="badge badge-primary">Badge</span>
    </div>
    {/* Content */}
  </div>
</div>
```

#### 3. Forms

```tsx
// Form input
<div className="form-control">
  <label className="label">
    <span className="label-text">Title</span>
  </label>
  <input
    type="text"
    className="input input-bordered"
    placeholder="Enter title"
    {...register('title')}
  />
  {errors.title && (
    <label className="label">
      <span className="label-text-alt text-error">{errors.title.message}</span>
    </label>
  )}
</div>

// Textarea
<div className="form-control">
  <label className="label">
    <span className="label-text">Description</span>
  </label>
  <textarea
    className="textarea textarea-bordered h-24"
    placeholder="Enter description"
    {...register('description')}
  />
</div>

// Select dropdown
<div className="form-control">
  <label className="label">
    <span className="label-text">Category</span>
  </label>
  <select className="select select-bordered" {...register('categoryType')}>
    <option value="">Select category</option>
    <option value="blog">Blog</option>
    <option value="news">News</option>
  </select>
</div>

// Checkbox
<div className="form-control">
  <label className="label cursor-pointer">
    <span className="label-text">Published</span>
    <input
      type="checkbox"
      className="checkbox checkbox-primary"
      {...register('published')}
    />
  </label>
</div>
```

#### 4. Badges

```tsx
// Basic badges
<span className="badge">Default</span>
<span className="badge badge-primary">Primary</span>
<span className="badge badge-secondary">Secondary</span>
<span className="badge badge-accent">Accent</span>

// Outline badges
<span className="badge badge-outline">Outline</span>
<span className="badge badge-primary badge-outline">Primary</span>

// Status badges
<span className="badge badge-success">Active</span>
<span className="badge badge-error">Deleted</span>
<span className="badge badge-warning">Pending</span>
```

#### 5. Modals

```tsx
// Modal with dialog element
<dialog className={`modal ${isOpen ? 'modal-open' : ''}`}>
  <div className="modal-box max-w-4xl">
    <h3 className="font-bold text-lg">Modal Title</h3>
    <p className="py-4">Modal content</p>
    <div className="modal-action">
      <button className="btn" onClick={onClose}>
        Close
      </button>
      <button className="btn btn-primary" onClick={onConfirm}>
        Confirm
      </button>
    </div>
  </div>
  <div className="modal-backdrop" onClick={onClose} />
</dialog>

// Full example from project
function MediaPreviewModal({ media, onClose }: Props) {
  return (
    <dialog className="modal modal-open">
      <div className="modal-box max-w-4xl">
        <button
          onClick={onClose}
          className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
        >
          <X className="w-4 h-4" />
        </button>
        <h3 className="font-bold text-lg mb-4">{media.fileName}</h3>
        <img src={media.url} alt={media.fileName} className="w-full" />
      </div>
      <div className="modal-backdrop bg-black/50" onClick={onClose} />
    </dialog>
  );
}
```

#### 6. Loading States

```tsx
// Spinner
<span className="loading loading-spinner loading-lg"></span>

// Progress bar
<progress className="progress progress-primary w-56"></progress>

// Skeleton loading (for tables)
<div className="space-y-4">
  {[...Array(5)].map((_, i) => (
    <div key={i} className="skeleton h-12 w-full"></div>
  ))}
</div>

// Card skeleton
<div className="card bg-base-100 shadow-xl">
  <div className="card-body space-y-4">
    <div className="skeleton h-4 w-1/2"></div>
    <div className="skeleton h-32 w-full"></div>
    <div className="skeleton h-4 w-1/3"></div>
  </div>
</div>
```

#### 7. Alerts/Toasts

Use `sonner` for toast notifications (not DaisyUI alerts):

```tsx
import { toast } from 'sonner';

// Success toast
toast.success('Operation completed successfully');

// Error toast
toast.error('Something went wrong');

// Custom toast
toast('Custom message', {
  description: 'Additional details',
  duration: 5000,
});
```

#### 8. Tabs

```tsx
<div role="tablist" className="tabs tabs-bordered">
  <a
    role="tab"
    className={`tab ${activeTab === 0 ? 'tab-active' : ''}`}
    onClick={() => setActiveTab(0)}
  >
    Tab 1
  </a>
  <a
    role="tab"
    className={`tab ${activeTab === 1 ? 'tab-active' : ''}`}
    onClick={() => setActiveTab(1)}
  >
    Tab 2
  </a>
</div>
```

#### 9. Dropdown Menu

```tsx
<div className="dropdown dropdown-end">
  <label tabIndex={0} className="btn btn-ghost btn-circle">
    <MoreVertical className="w-5 h-5" />
  </label>
  <ul
    tabIndex={0}
    className="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52"
  >
    <li><a onClick={handleEdit}>Edit</a></li>
    <li><a onClick={handleDelete}>Delete</a></li>
  </ul>
</div>
```

### DaisyUI Theme

The project uses DaisyUI's default theme system. Common theme classes:

- `bg-base-100`, `bg-base-200`, `bg-base-300` - Background colors
- `text-base-content` - Text color
- `border-base-300` - Border color

### Icons with Lucide React

Use Lucide React for icons:

```tsx
import { Save, Edit, Trash2, Plus, X, FolderOpen } from 'lucide-react';

<button className="btn btn-primary">
  <Save className="w-4 h-4" />
  Save
</button>
```

**Icon Sizing Guidelines:**
- Small buttons: `w-4 h-4`
- Normal buttons: `w-5 h-5`
- Large buttons: `w-6 h-6`
- Section headers: `w-5 h-5` or `w-6 h-6`

## Form Handling

### React Hook Form + Zod

The project uses React Hook Form with Zod for validation.

#### Define Schema

```tsx
// schemas/category.schema.ts
import { z } from 'zod';

export const categoryFormSchema = z.object({
  displayName: z.string().min(1, 'Display name is required'),
  categoryType: z.enum(['Blog', 'News', 'Portfolio']),
  tagNames: z.array(z.string()).optional(),
  translations: z.array(
    z.object({
      id: z.string().uuid().optional(),
      languageCode: z.string(),
      displayName: z.string().min(1, 'Translation name is required'),
    })
  ).optional(),
  rowVersion: z.number().default(0),
});

export type CategoryFormData = z.infer<typeof categoryFormSchema>;
```

#### Use in Component

```tsx
import { useForm, Controller } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { categoryFormSchema, type CategoryFormData } from '@/schemas/category.schema';

export default function CategoryForm() {
  const {
    register,
    handleSubmit,
    control,
    reset,
    watch,
    formState: { errors, isSubmitting },
  } = useForm<CategoryFormData>({
    resolver: zodResolver(categoryFormSchema),
    defaultValues: {
      displayName: '',
      categoryType: 'Blog',
      tagNames: [],
      translations: [],
      rowVersion: 0,
    },
  });

  const onSubmit = async (data: CategoryFormData) => {
    try {
      // Submit logic
      const response = await authenticatedFetch(
        getApiUrl('/categories'),
        token,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(data),
        }
      );
      
      if (response.ok) {
        toast.success('Category created successfully');
        navigate('/admin/categories');
      } else {
        toast.error('Failed to create category');
      }
    } catch (error) {
      console.error('Error:', error);
      toast.error('An error occurred');
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <div className="form-control">
        <label className="label">
          <span className="label-text">Display Name</span>
        </label>
        <input
          type="text"
          className="input input-bordered"
          {...register('displayName')}
        />
        {errors.displayName && (
          <label className="label">
            <span className="label-text-alt text-error">
              {errors.displayName.message}
            </span>
          </label>
        )}
      </div>

      <button
        type="submit"
        className="btn btn-primary"
        disabled={isSubmitting}
      >
        {isSubmitting ? 'Saving...' : 'Save'}
      </button>
    </form>
  );
}
```

### Field Arrays

For dynamic form fields (e.g., translations, tags):

```tsx
import { useFieldArray } from 'react-hook-form';

const { fields, append, remove } = useFieldArray({
  control,
  name: 'translations',
});

// Add new field
const addTranslation = () => {
  append({
    languageCode: '',
    displayName: '',
  });
};

// Render fields
{fields.map((field, index) => (
  <div key={field.id} className="border p-4 rounded-lg">
    <div className="flex justify-between items-center mb-2">
      <h3>Translation {index + 1}</h3>
      <button
        type="button"
        className="btn btn-ghost btn-sm btn-circle"
        onClick={() => remove(index)}
      >
        <X className="w-4 h-4" />
      </button>
    </div>
    
    <div className="form-control">
      <input
        type="text"
        className="input input-bordered"
        {...register(`translations.${index}.displayName`)}
      />
    </div>
  </div>
))}

<button
  type="button"
  className="btn btn-outline"
  onClick={addTranslation}
>
  <Plus className="w-4 h-4" />
  Add Translation
</button>
```

### Controlled Components

For complex inputs (e.g., rich text editor):

```tsx
import { Controller } from 'react-hook-form';

<Controller
  name="content"
  control={control}
  render={({ field }) => (
    <RichTextEditor
      value={field.value}
      onChange={field.onChange}
    />
  )}
/>
```

## State Management

### Local State with useState

For component-specific state:

```tsx
const [isOpen, setIsOpen] = useState(false);
const [loading, setLoading] = useState(false);
const [data, setData] = useState<Data[]>([]);
```

### Context for Shared State

For authentication and layout state:

```tsx
// AuthContext.tsx
import { createContext, useContext, useState, useEffect } from 'react';
import Keycloak from 'keycloak-js';

interface AuthContextType {
  keycloak: Keycloak | null;
  token: string | null;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [keycloak, setKeycloak] = useState<Keycloak | null>(null);
  const [token, setToken] = useState<string | null>(null);
  
  // Initialize Keycloak
  useEffect(() => {
    // Initialization logic
  }, []);

  return (
    <AuthContext.Provider value={{ keycloak, token, isAuthenticated: !!token }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
}
```

### URL State with React Router

For pagination, filters, etc.:

```tsx
import { useSearchParams } from 'react-router-dom';

function ListPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  
  const page = parseInt(searchParams.get('page') || '1');
  const search = searchParams.get('search') || '';
  
  const handlePageChange = (newPage: number) => {
    setSearchParams({ page: newPage.toString(), search });
  };
  
  return (
    // Component JSX
  );
}
```

## Data Fetching

### Authenticated Fetch

Use the project's authenticated fetch utility:

```tsx
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';

function Component() {
  const { token, keycloak } = useAuth();
  
  const fetchData = async () => {
    try {
      const response = await authenticatedFetch(
        getApiUrl('/categories'),
        token,
        { cache: 'no-store' },
        keycloak || undefined
      );
      
      if (response && response.ok) {
        const data = await response.json();
        return data;
      }
    } catch (error) {
      console.error('Fetch error:', error);
      toast.error('Failed to fetch data');
    }
  };
  
  useEffect(() => {
    fetchData();
  }, []);
}
```

### GraphQL with Apollo Client

For GraphQL queries:

```tsx
import { useQuery, gql } from '@apollo/client';

const GET_POSTS = gql`
  query GetPosts($limit: Int!, $offset: Int!) {
    posts(limit: $limit, offset: $offset) {
      id
      title
      slug
      createdAt
    }
  }
`;

function PostsList() {
  const { loading, error, data } = useQuery(GET_POSTS, {
    variables: { limit: 10, offset: 0 },
  });
  
  if (loading) return <LoadingSkeleton />;
  if (error) return <ErrorMessage error={error} />;
  
  return (
    <div>
      {data.posts.map((post) => (
        <PostCard key={post.id} post={post} />
      ))}
    </div>
  );
}
```

## Routing

### React Router v7

```tsx
import { BrowserRouter, Routes, Route } from 'react-router-dom';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/admin" element={<AdminLayout />}>
          <Route index element={<DashboardPage />} />
          <Route path="categories" element={<CategoriesPage />} />
          <Route path="categories/create" element={<CreateCategoryPage />} />
          <Route path="categories/edit/:id" element={<EditCategoryPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}
```

### Navigation

```tsx
import { useNavigate, useParams } from 'react-router-dom';

function Component() {
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  
  const handleSave = async () => {
    // Save logic
    navigate('/admin/categories');
  };
  
  const handleCancel = () => {
    navigate(-1); // Go back
  };
}
```

## Authentication

### Keycloak Integration

```tsx
import { useAuth } from '@/auth/AuthContext';

function ProtectedComponent() {
  const { isAuthenticated, keycloak, token } = useAuth();
  
  if (!isAuthenticated) {
    return <div>Please log in</div>;
  }
  
  const handleLogout = () => {
    keycloak?.logout();
  };
  
  return (
    <div>
      {/* Protected content */}
      <button onClick={handleLogout} className="btn">
        Logout
      </button>
    </div>
  );
}
```

## Code Examples

### Complete CRUD Page Example

```tsx
import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { Plus, Edit, Trash2 } from 'lucide-react';
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';
import type { CategoryModel } from '@/domains/category';

export default function CategoriesPage() {
  const navigate = useNavigate();
  const { token, keycloak } = useAuth();
  const [categories, setCategories] = useState<CategoryModel[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchCategories();
  }, []);

  const fetchCategories = async () => {
    setLoading(true);
    try {
      const response = await authenticatedFetch(
        getApiUrl('/categories'),
        token,
        { cache: 'no-store' },
        keycloak || undefined
      );
      
      if (response && response.ok) {
        const data = await response.json();
        setCategories(data.data || []);
      }
    } catch (error) {
      console.error('Failed to fetch categories:', error);
      toast.error('Failed to load categories');
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this category?')) {
      return;
    }

    try {
      const response = await authenticatedFetch(
        getApiUrl(`/categories/${id}`),
        token,
        { method: 'DELETE' },
        keycloak || undefined
      );

      if (response && response.ok) {
        toast.success('Category deleted successfully');
        fetchCategories();
      } else {
        toast.error('Failed to delete category');
      }
    } catch (error) {
      console.error('Delete error:', error);
      toast.error('An error occurred');
    }
  };

  if (loading) {
    return (
      <div className="space-y-4">
        {[...Array(5)].map((_, i) => (
          <div key={i} className="skeleton h-12 w-full"></div>
        ))}
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">Categories</h1>
        <button
          onClick={() => navigate('/admin/categories/create')}
          className="btn btn-primary"
        >
          <Plus className="w-4 h-4" />
          New Category
        </button>
      </div>

      <div className="grid gap-4">
        {categories.map((category) => (
          <div
            key={category.id}
            className="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow"
          >
            <div className="card-body">
              <div className="flex justify-between items-center">
                <div>
                  <h2 className="card-title">{category.displayName}</h2>
                  <p className="text-sm text-base-content/70">
                    {category.categoryType}
                  </p>
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() => navigate(`/admin/categories/edit/${category.id}`)}
                    className="btn btn-ghost btn-sm"
                  >
                    <Edit className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => handleDelete(category.id)}
                    className="btn btn-ghost btn-sm text-error"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </div>
              {category.tags && category.tags.length > 0 && (
                <div className="flex gap-2 mt-2">
                  {category.tags.map((tag) => (
                    <span key={tag.id} className="badge badge-outline">
                      {tag.name}
                    </span>
                  ))}
                </div>
              )}
            </div>
          </div>
        ))}
      </div>

      {categories.length === 0 && (
        <div className="text-center py-12">
          <p className="text-base-content/70">No categories found</p>
          <button
            onClick={() => navigate('/admin/categories/create')}
            className="btn btn-primary mt-4"
          >
            Create your first category
          </button>
        </div>
      )}
    </div>
  );
}
```

## Performance Best Practices

1. **Memoization**: Use `useMemo` and `useCallback` for expensive computations
2. **Code Splitting**: Use dynamic imports for large components
3. **Lazy Loading**: Implement lazy loading for images and routes
4. **Debouncing**: Debounce search inputs and API calls
5. **Virtualization**: Use virtual lists for long lists

## Accessibility

1. **Semantic HTML**: Use appropriate HTML elements
2. **ARIA Labels**: Add aria labels for screen readers
3. **Keyboard Navigation**: Ensure all interactive elements are keyboard accessible
4. **Focus Management**: Manage focus for modals and dynamic content
5. **Color Contrast**: Maintain WCAG AA contrast ratios

## Summary

Key takeaways for React development in My-CMS:

1. **Use DaisyUI components** - Consistent UI with utility classes
2. **React Hook Form + Zod** - Type-safe form validation
3. **Authenticated fetch** - Use project's auth utilities
4. **Toast notifications** - Use Sonner for user feedback
5. **Lucide React icons** - Consistent icon system
6. **Type safety** - Leverage TypeScript strictly
7. **Component composition** - Build reusable components
8. **Error handling** - Handle errors gracefully with user feedback
