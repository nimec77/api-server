# Todo API Server

A modern, async REST API server built with Rust, Axum, and SQLite for managing todo items. This project demonstrates best practices in Rust async programming, database management, and API design.

## Features

- **RESTful API**: Full CRUD operations for todo items
- **Async Architecture**: Built with Tokio runtime for high-performance async operations
- **Database Integration**: SQLite with SQLx for type-safe database queries
- **Automatic Migrations**: Database schema management with SQLx migrations
- **CORS Support**: Cross-origin resource sharing enabled
- **Structured Logging**: Comprehensive logging with tracing
- **Health Checks**: Built-in health and readiness endpoints
- **Error Handling**: Robust error handling with proper HTTP status codes

## Tech Stack

- **Runtime**: Tokio (async runtime)
- **Web Framework**: Axum
- **Database**: SQLite with SQLx
- **Serialization**: Serde
- **Logging**: Tracing + Tracing Subscriber
- **HTTP Middleware**: Tower HTTP (CORS, tracing)

## Prerequisites

- Rust 1.85+ (2024 edition)
- Cargo (Rust package manager)

## Installation & Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd api-server
   ```

2. **Install dependencies and build**
   ```bash
   cargo build
   ```

3. **Run the server**
   ```bash
   cargo run
   ```

The server will start on `127.0.0.1:3000` by default.

## Configuration

The application can be configured using environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `BIND_ADDR` | `127.0.0.1:3000` | Server bind address |
| `DATABASE_URL` | `sqlite:db.sqlite` | SQLite database connection string |
| `RUST_LOG` | `sqlx=debug,tower_http=debug,info` | Logging level configuration |

### Example Configuration

```bash
export BIND_ADDR="0.0.0.0:8080"
export DATABASE_URL="sqlite:./data/todos.db"
export RUST_LOG="info"
cargo run
```

## API Endpoints

### Health Checks

- `GET /alive` - Simple health check
- `GET /ready` - Database connectivity check

### Todo Management

All todo endpoints are prefixed with `/v1`:

#### List Todos
```http
GET /v1/todos
```

**Response:**
```json
[
  {
    "id": 1,
    "body": "Buy groceries",
    "completed": false,
    "created_at": "2024-01-15T10:30:00",
    "updated_at": "2024-01-15T10:30:00"
  }
]
```

#### Get Todo by ID
```http
GET /v1/todos/{id}
```

#### Create Todo
```http
POST /v1/todos
Content-Type: application/json

{
  "body": "Learn Rust async programming"
}
```

#### Update Todo
```http
PUT /v1/todos/{id}
Content-Type: application/json

{
  "body": "Learn Rust async programming",
  "completed": true
}
```

#### Delete Todo
```http
DELETE /v1/todos/{id}
```

## Database Schema

The application uses a SQLite database with the following schema:

```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    body TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

Database migrations are automatically applied on startup.

## Project Structure

```
api-server/
├── src/
│   ├── main.rs          # Application entry point
│   ├── api.rs           # API endpoint handlers
│   ├── router.rs        # Axum router configuration
│   ├── todo.rs          # Todo model and database operations
│   └── error.rs         # Error handling
├── migrations/
│   └── 20250827100440_todos.sql  # Database migration
├── Cargo.toml           # Dependencies and project metadata
└── README.md           # This file
```

## Development

### Running Tests

```bash
cargo test
```

### Running with Logging

```bash
RUST_LOG=debug cargo run
```

### Database Operations

The application automatically handles database setup and migrations. The SQLite database file (`db.sqlite`) will be created automatically on first run.

## Example Usage

### Using curl

```bash
# Create a todo
curl -X POST http://localhost:3000/v1/todos \
  -H "Content-Type: application/json" \
  -d '{"body": "Build a Rust API"}'

# List all todos
curl http://localhost:3000/v1/todos

# Update a todo
curl -X PUT http://localhost:3000/v1/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"body": "Build a Rust API", "completed": true}'

# Delete a todo
curl -X DELETE http://localhost:3000/v1/todos/1
```

### Using JavaScript/Fetch

```javascript
// Create a todo
const response = await fetch('http://localhost:3000/v1/todos', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    body: 'Learn Rust async programming'
  })
});

const todo = await response.json();
console.log(todo);
```

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK` - Successful operations
- `201 Created` - Resource created successfully
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server errors

## Performance Considerations

- **Connection Pooling**: SQLx connection pool for efficient database connections
- **Async Operations**: All I/O operations are async for better concurrency
- **Structured Logging**: Efficient logging with configurable levels
- **CORS Optimization**: Configured for cross-origin requests

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Database operations powered by [SQLx](https://github.com/launchbadge/sqlx)
- Async runtime provided by [Tokio](https://tokio.rs/)
