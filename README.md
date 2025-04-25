# Tauri V2 OAuth Login Example

A comprehensive example demonstrating OAuth2 authentication flow in Tauri V2 applications, featuring Google and GitHub login integration. This project showcases a complete client-server authentication implementation using Rust (Tauri) and Vue 3 + TypeScript.

## Features

- 🔐 OAuth2 authentication with multiple providers:
  - Google OAuth2
  - GitHub OAuth2
- 🌐 Complete authentication flow simulation:
  - OAuth server implementation
  - Authorization code handling
  - Token exchange
- 💾 Persistent session management using Tauri Store V2
- 🔒 Secure token handling

## Screenshot
![image](https://github.com/pollosssss/tauri2-login-example/blob/main/screenshot.png)

## Getting Started

### Prerequisites

- Node.js (v16 or later)
- Rust (latest stable)
- VS Code with recommended extensions:
  - Volar (Vue)
  - rust-analyzer
  - Tauri

### Setup OAuth Credentials

1. **Google OAuth Setup**
   - Go to [Google Cloud Console](https://console.cloud.google.com/)
   - Create a new project
   - Enable OAuth2 API
   - Create OAuth credentials
   - Set redirect URI to: `http://localhost:8000`
   - Copy client ID and secret

2. **GitHub OAuth Setup**
   - Go to [GitHub Developer Settings](https://github.com/settings/developers)
   - Create new OAuth App
   - Set Authorization callback URL to: `http://localhost:8000`
   - Copy client ID and secret

3. **Configure OAuth Credentials**
   Update `src-tauri/oauth_config.json`:
   ```json
   {
     "google": {
       "client_id": "YOUR_GOOGLE_CLIENT_ID",
       "client_secret": "YOUR_GOOGLE_CLIENT_SECRET",
       "auth_url": "https://accounts.google.com/o/oauth2/v2/auth",
       "token_url": "https://oauth2.googleapis.com/token",
       "user_info_url": "https://www.googleapis.com/oauth2/v3/userinfo",
       "scope": "email profile openid"
     },
     "github": {
       "client_id": "YOUR_GITHUB_CLIENT_ID",
       "client_secret": "YOUR_GITHUB_CLIENT_SECRET",
       "auth_url": "https://github.com/login/oauth/authorize",
       "token_url": "https://github.com/login/oauth/access_token",
       "user_info_url": "https://api.github.com/user",
       "scope": "user:email read:user"
     }
   }
   ```

### Installation

```bash
# Install dependencies
npm install

# Start development
npm run tauri dev

# Build for production
npm run tauri build
```

## Authentication Flow

1. **Initial OAuth Request**
   - User clicks login button
   - Application starts local OAuth server (ports 8000-8002)
   - Opens browser with OAuth provider's authorization URL

2. **Authorization Code Reception**
   ```rust
   // Rust code handling OAuth callback
   let oauth_config = tauri_plugin_oauth::OauthConfig {
       ports: Some(vec![8000, 8001, 8002]),
       response: Some("OAuth process completed".into()),
   };
   
   tauri_plugin_oauth::start_with_config(oauth_config, |url| {
       // Extract authorization code from URL
       let code = extract_code_from_url(url);
       // Process the code...
   })
   ```

3. **Token Exchange**
   - Application exchanges authorization code for access token
   - Tokens are securely stored using Tauri Store V2

4. **User Session Management**
   ```typescript
   // TypeScript code for session management
   const store = await load('user-store.json', { autoSave: true });
   await store.set('user', userInfo);
   ```

## Acknowledgments

Thanks to [tauri-plugin-oauth](https://github.com/FabianLars/tauri-plugin-oauth).