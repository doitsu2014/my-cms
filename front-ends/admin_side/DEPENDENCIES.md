# Dependencies Installation Guide

This document provides information about the dependencies in the admin_side module.

## Installation

To install all dependencies, run:

```bash
cd admin_side
pnpm install
```

## Added Dependencies

### Core Dependencies (Production)

#### Authentication
- **keycloak-js** (^26.0.7) - Official Keycloak JavaScript adapter for browser applications
  - Implements Authorization Code Flow with PKCE
  - Handles token refresh automatically
  - Provides user info and session management
  - See Phase 8 implementation in PHASE_8_10_IMPLEMENTATION.md

#### GraphQL & Data Layer
- **@apollo/client** (^3.12.6) - GraphQL client for React applications
  - Configured for my-cms backend integration
  - Automatic Keycloak Bearer token injection
  - In-memory caching
- **graphql** (^16.10.0) - GraphQL query language implementation
- **graphql-tag** (^2.12.6) - GraphQL query parser

#### Rich Text Editor
- **quill** (^2.0.3) - Modern WYSIWYG editor
- **quill-html-edit-button** (^3.0.0) - HTML editing plugin for Quill
- **quill-resize-image** (^1.0.5) - Image resizing plugin for Quill
- **quill-table-better** (^1.1.6) - Enhanced table support for Quill
- **quill-toggle-fullscreen-button** (^0.1.3) - Fullscreen mode for Quill

#### UI & Styling
- **daisyui** (^5.0.0) - Tailwind CSS component library
- **tailwindcss** (^4.0.12) - Utility-first CSS framework
- **lucide-react** (^0.476.0) - Icon library

#### Utilities
- **highlight.js** (^11.11.1) - Syntax highlighting for code blocks
- **slugify** (^1.6.6) - URL-friendly slug generation
- **react-router-dom** (^7.9.0) - Client-side routing for React

### Development Dependencies

#### Styling Tools
- **@tailwindcss/typography** (^0.5.16) - Typography plugin for Tailwind CSS
- **postcss** (^8.5.1) - CSS transformation tool
- **autoprefixer** (^10.4.20) - Automatically add vendor prefixes

#### Type Definitions
- **@types/jsonwebtoken** (^9.0.7) - TypeScript types for JWT

## Configuration Files

The following configuration files were created:

### tailwind.config.ts
Configures Tailwind CSS with:
- DaisyUI plugin for components
- Typography plugin for rich text
- Light and dark themes

### postcss.config.mjs
Configures PostCSS with:
- Tailwind CSS processing
- Autoprefixer for browser compatibility

### .env.example
Template for environment variables:
- GraphQL API endpoint
- Cache configuration
- Upload endpoints

## Usage Notes

### Quill Editor Plugins
The Quill editor has been configured with several plugins:
- HTML editing for advanced users
- Image resizing for better UX
- Enhanced table support
- Fullscreen mode for distraction-free writing

### DaisyUI Components
DaisyUI provides pre-built components that follow:
- Tailwind CSS utility classes
- Accessible design patterns
- Customizable themes

### GraphQL Client
Apollo Client is configured to:
- Connect to the GraphQL backend
- Cache query results
- Handle mutations and subscriptions

## Next Steps

After installing dependencies:
1. Create a `.env.local` file based on `.env.example`
2. Configure the GraphQL API URL
3. Start the development server: `npm run dev`

## Troubleshooting

### Quill Plugins Not Working
If Quill plugins don't load:
1. Check that all quill-* packages are installed
2. Verify plugin imports in component files
3. Ensure Quill CSS is imported

### Tailwind Not Applying
If Tailwind styles don't apply:
1. Verify `tailwind.config.ts` content paths
2. Check that PostCSS is configured
3. Import Tailwind CSS in your main CSS file

### GraphQL Connection Issues
If GraphQL queries fail:
1. Verify `MY_CMS_API_URL` in `.env.local`
2. Check that the backend server is running
3. Verify the GraphQL endpoint is accessible

## Security Notes

- All dependencies have been scanned for vulnerabilities
- No known security issues found
- Keep dependencies updated regularly
- Review dependency updates before installing

## Version Compatibility

These dependencies are compatible with:
- React 19.x
- TypeScript 5.9.x
- Node.js 18.x or higher
- Rsbuild 1.5.x

## Additional Resources

- [Apollo Client Documentation](https://www.apollographql.com/docs/react/)
- [Quill Editor Documentation](https://quilljs.com/)
- [DaisyUI Documentation](https://daisyui.com/)
- [Tailwind CSS Documentation](https://tailwindcss.com/)
- [React Router Documentation](https://reactrouter.com/)
