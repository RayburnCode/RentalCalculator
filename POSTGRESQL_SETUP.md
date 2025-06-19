<!-- @format -->

# PostgreSQL Setup for Rental Calculator

This project has been converted from SQLite to PostgreSQL while maintaining SQLx compatibility.

## Prerequisites

1. Install PostgreSQL on your system
2. Create a database named `rental_calculator`

## Database Setup

### Option 1: Using Docker (Recommended)

```bash
docker run --name rental-calculator-db \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=rental_calculator \
  -p 5432:5432 \
  -d postgres:15
```

### Option 2: Local PostgreSQL Installation

1. Install PostgreSQL
2. Create a database:

```sql
CREATE DATABASE rental_calculator;
```

## Configuration

1. Copy the `.env` file and adjust the database URL if needed:

```
DATABASE_URL=postgresql://postgres:password@localhost:5432/rental_calculator
```

2. Run database migrations:

```bash
# If you have sqlx-cli installed
sqlx migrate run

# Or manually execute the migration file
psql -d rental_calculator -f migrations/001_initial_setup.sql
```

## Key Changes from SQLite to PostgreSQL

1. **Database Types**: Changed from SQLite to PostgreSQL types

   - `INTEGER PRIMARY KEY` → `SERIAL PRIMARY KEY`
   - Updated connection strings and pool types

2. **SQL Syntax**: Updated SQL statements for PostgreSQL compatibility

   - Changed `INSERT ... SELECT` to `INSERT ... VALUES`
   - Updated `ON CONFLICT` clauses
   - Added foreign key constraints

3. **Dependencies**: Updated Cargo.toml files

   - Changed `axum_session_sqlx` features from `sqlite` to `postgres`
   - Updated SQLx features to include PostgreSQL support

4. **Connection Management**:
   - Environment variable support for database URL
   - Improved error handling for database connections

## Running the Application

1. Ensure PostgreSQL is running and the database is created
2. Set up the environment variables (`.env` file)
3. Run the migrations
4. Start the application:

```bash
cargo run
```

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string (default: `postgresql://postgres:password@localhost:5432/rental_calculator`)
