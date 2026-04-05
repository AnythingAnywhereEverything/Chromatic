# Polyglot Template
This project is a template for a polyglot project, which can be used to create projects that support multiple programming languages. The template includes a basic structure for organizing code and resources, as well as a sample implementation in Rust, Elixir, Python, and Next+React with Docker and NGINX.

## Project Structure
The project is organized into the following directories:
- /backend: Contains the backend code for the project, including implementations in Rust, Elixir, and Python.
- /frontend: Contains the frontend code for the project, implemented in Next+React.

Each language contains its own directory with the necessary files and configurations for that language. The project also includes a Dockerfile and NGINX configuration for deploying the application.

## Prerequisites
Before you can run the project, make sure you have the following installed on your machine:
- Docker: You can download and install Docker from [here](https://www.docker.com/get-started).
- Git: You can download and install Git from [here](https://git-scm.com/downloads).
- Redis: You can download and install Redis from [here](https://redis.io/download).
- Node.js and npm: You can download and install Node.js and npm from [here](https://nodejs.org/).
- Rust: You can download and install Rust from [here](https://www.rust-lang.org/tools/install).
- Elixir: You can download and install Elixir from [here](https://elixir-lang.org/install.html).
- Python: You can download and install Python from [here](https://www.python.org/downloads/).
- PostgreSQL: You can download and install PostgreSQL from [here](https://www.postgresql.org/download/).
- NGINX: You can download and install NGINX from [here](https://nginx.org/en/download.html).

## Getting Started
To get started with the project, follow these steps:
``` shell
# Clone the repository:
   git clone <repository-url>

# Navigate to the project directory:
   cd polyglot-template

# Build and run docker compose:
    docker compose up --build

# Run the docker compose command to start the application:
    docker compose up

# If you don't have PostgreSQL, Redis, or NGINX installed, you can run them using Docker Compose Full:
    # Build and run docker compose full:
    docker compose -f docker-compose-full.yml up --build
    
    # Run the docker compose full command to start the application with PostgreSQL, Redis, and NGINX:
    docker compose -f docker-compose-full.yml up
```
Access the application by navigating to `http://localhost` in your web browser. You should see the frontend interface, and you can interact with the backend services implemented in Rust, Elixir, and Python.

## Contributing
Contributions to this project are welcome! If you have any improvements or new features to add, please feel free to submit a pull request. Make sure to follow the existing code style and include tests for any new functionality.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.