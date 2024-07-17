# User Management Service

## Description

The User Management Service is a Rust-based application designed to manage user information through a gRPC API. This service allows for the creation, updating, deletion, and retrieval of user data, leveraging PostgreSQL as the database backend. The application is containerized using Docker, making it easy to deploy and manage.

## Prerequisites

To set up and run this project, you need the following:

- Docker
- Docker Compose
- Rust and Cargo
- PostgreSQL

## Technologies Used

- **Rust**: The primary programming language used to develop the service.
- **Docker**: Used for containerizing the application.
- **PostgreSQL**: The database system for storing user information.
- **gRPC**: Communication framework used to build the API.
- **Tonic**: A Rust implementation of gRPC.
- **SQLx**: A Rust SQL toolkit and ORM.
- **Color Eyre**: Error handling in Rust.
- **Tokio**: An asynchronous runtime for Rust.
- **Tracing Subscriber**: For logging and diagnostics.
- **Serde**: Serialization and deserialization framework.

## Setup and Running the Service

### Docker Setup

The `docker-compose.yml` file defines the services required for the application:

- `db`: A PostgreSQL database service.
- `web`: The Rust-based web service that interacts with the database.

To start the services, run:

```bash
docker-compose up --build
```

### Running Migrations

The service uses SQLx for database interactions, including running migrations. To run migrations, follow these steps:

1. Ensure the database container is running:

    ```bash
    docker-compose up db
    ```

2. Run the migrations using SQLx CLI:

    ```bash
    sqlx migrate run
    ```

    This command will apply all the pending migrations to the database specified in the `DATABASE_URL` environment variable.

### Building the Proto Files

The `build.rs` file is used to compile the protobuf definitions. The necessary configurations and compilation steps are defined to ensure the proto files are correctly built and integrated into the Rust application.

### Main Application

The main application is responsible for setting up the gRPC server, initializing the database connection, running migrations, and starting the service. Environment variables are used to configure the server address and database URL.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.