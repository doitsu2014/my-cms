# Admin Side Module

## Overview
The `admin_side` module is a micro-frontend application built with **Rsbuild** and **React 19** that provides administrative features for the blog platform. It uses **Module Federation 2.0** to integrate seamlessly with the shell application.

## Current Status
🚧 **Under Development** - Migration in progress from [doitsu2014/my-blogs](https://github.com/doitsu2014/my-blogs)

## Planned Features

### Categories Management
- View all blog categories
- Create new categories
- Edit existing categories
- Delete categories
- Automatic slug generation

### Blogs Management
- View all blog posts
- Create new blog posts with rich text editor (Quill)
- Edit existing blog posts
- Delete blog posts
- Upload and manage images
- Tag management
- Category assignment
- Publish/Draft status control

### Admin Dashboard
- Overview statistics
- Quick access to management features
- Recent activities

### Authentication & Authorization
- Secure login/logout
- Session management
- Protected admin routes

## Technology Stack

- **Framework**: React 19.1.x
- **Build Tool**: Rsbuild 1.5.x
- **Language**: TypeScript 5.9.x
- **Module Federation**: @module-federation/enhanced 0.21.6
- **UI Framework**: DaisyUI v5 + Tailwind CSS (planned)
- **Rich Text Editor**: Quill 2.0 (planned)
- **Data Layer**: Apollo Client + GraphQL (planned)
- **Authentication**: next-auth v5 (planned, may be adapted)
- **Icons**: lucide-react (planned)

## Architecture

This module is designed as a **Remote Module** in the Module Federation architecture:
- **Port**: 3002 (planned)
- **Exposes**: Admin components for consumption by the shell app
- **Shared**: React, React-DOM, and other common dependencies

## Migration Plan

See the [MIGRATION_PLAN.md](../MIGRATION_PLAN.md) in the root directory for the complete migration strategy from the old Next.js platform to this micro-frontend architecture.

### Migration Phases
1. ✅ Repository Analysis & Setup
2. 🚧 Domain Models & Infrastructure
3. ⏳ Dependencies Installation
4. ⏳ Admin Layout & Context
5. ⏳ Categories Management
6. ⏳ Blogs Management
7. ⏳ Admin Dashboard
8. ⏳ Authentication & Middleware
9. ⏳ Configuration & Styling
10. ⏳ Module Federation Configuration
11. ⏳ Testing & Validation
12. ⏳ Documentation

Legend: ✅ Complete | 🚧 In Progress | ⏳ Pending

## Development

### Prerequisites
- Node.js (LTS version)
- pnpm (package manager)

### Installation
```bash
cd admin_side
pnpm install
```

### Development Server
```bash
pnpm dev
```
The development server will start on port 3002 (once configured).

### Build
```bash
pnpm build
```

### Linting
```bash
pnpm lint
```

### Format Code
```bash
pnpm format
```

## Environment Variables

(To be documented after migration)

```env
# GraphQL API
GRAPHQL_API_URL=http://localhost:4000/graphql

# Authentication
NEXTAUTH_URL=http://localhost:3002
NEXTAUTH_SECRET=your-secret-key

# Upload/Storage
UPLOAD_URL=http://localhost:4000/upload
```

## Document metadata policy

The admin document starts with the product title `My-CMS Admin` and a
`noindex, nofollow` robots directive. Route navigation refines the title for login,
dashboard, list, create, and edit tasks while preserving that private robots policy.
The admin app intentionally emits no public-site canonical, Open Graph, or Twitter
metadata.

## Project Structure

```
admin_side/
├── src/
│   ├── app/                    # Application pages
│   │   └── admin/             # Admin features (to be migrated)
│   │       ├── blogs/         # Blog management
│   │       ├── categories/    # Category management
│   │       ├── components/    # Shared admin components
│   │       └── page.tsx       # Admin dashboard
│   ├── domains/               # Domain models (to be added)
│   ├── infrastructure/        # Infrastructure layer (to be added)
│   ├── App.tsx               # Main app component
│   └── index.tsx             # Entry point
├── rsbuild.config.ts         # Rsbuild configuration
├── package.json              # Dependencies
└── README.md                 # This file
```

## Integration with Shell App

Once migration is complete, the shell app will load this module as a remote:

```typescript
// In shell/rsbuild.config.ts
remotes: {
  admin_side: 'admin_side@http://localhost:3002/mf-manifest.json'
}
```

Then import admin components in the shell:
```typescript
import AdminDashboard from 'admin_side/AdminDashboard';
```

## Contributing

This module is being actively developed. Please coordinate with the development team before making changes.

## Related Documentation

- [Migration Plan](../MIGRATION_PLAN.md) - Detailed migration strategy
- [Main Project README](../README.md) - Overall project documentation
- [Module Federation Guide](https://module-federation.io/) - Module Federation documentation

## License

Private project - All rights reserved

---

**Status**: 🚧 Under Development  
**Version**: 0.1.0  
**Last Updated**: 2025-12-18
