# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

This is a full-stack Rust web application with a PostgreSQL database, structured as a Cargo workspace with two main components:

- **Backend** (`backend/`): Axum-based REST API server with PostgreSQL integration via SQLx
- **Frontend** (`frontend/`): Yew-based WebAssembly frontend served via Trunk

The project includes Docker containerization, Helm charts for Kubernetes deployment, and Terraform infrastructure configuration.

## Development Commands

### Backend Development
```bash
cd backend
cargo watch -x run
```
The backend serves on the port/address specified by `SERVER_PORT` and `SERVER_ADDRESS` environment variables.

### Frontend Development  
```bash
cd frontend
trunk serve
```
The frontend serves on port 8082 (configured in `Trunk.toml`).

### Database Setup
```bash
# Start PostgreSQL via Docker Compose
docker-compose -f docker-compose_2.yml up -d

# Connect to database
psql -h localhost -p 5432 -U postgres -d postgres
# Password: keyoarbcat

# Stop database
docker-compose down
```

### Testing
```bash
# Run tests for specific module
cargo test
```

### Code Formatting
The project uses rustfmt with 2-space indentation (configured in `rustfmt.toml`):
```bash
cargo fmt
```

## Environment Configuration

### Backend Environment Variables
- `SERVER_PORT`: Backend server port
- `SERVER_ADDRESS`: Backend server address  
- `DATABASE_URL`: PostgreSQL connection string

### Development vs Production
- Development: Uses `dotenvy` to load `.env` file
- Production: Reads environment variables directly from system

## Database Architecture

- Uses SQLx for database operations with compile-time query verification
- PostgreSQL database with initialization scripts in `backend/database_init/`
- User entity with CRUD operations via repository pattern

## API Structure

The backend exposes:
- `GET /` - Health check endpoint
- `GET /hello` - Hello endpoint
- `GET /users` - Get all users
- `POST /users` - Create user  
- `GET /users/:email` - Get user by email
- `PUT /users/:email` - Update user
- `DELETE /users/:email` - Delete user

## Frontend Architecture

- Yew framework with component-based architecture
- Client-side routing with yew-router
- Context providers for state management
- API integration with backend via gloo-net

## Key Components

- **Math modules**: Basic arithmetic operations (`backend/src/math/`)
- **Quadratic formula solver**: Mathematical computation example
- **User management**: Full CRUD operations for user entities
- **CORS configuration**: Permissive CORS setup for development

## Docker & Deployment

- Multi-stage Dockerfiles for both frontend and backend
- Helm charts in `helm_charts/` for Kubernetes deployment
- Infrastructure as Code with Terraform in `infrastructure/`

## Development Notes

- Uses trunk-version 0.21.14 for frontend builds
- CORS is configured permissively for development
- Database password is `keyoarbcat` for development environment