# Chromatic
Chromatic is a social media platform made for students (academically), though originally intended for a broader audience. The project is built using a polyglot approach, leveraging multiple programming languages and frameworks to create a scalable application.

## Disclaimer
This project is for educational purposes only and will not be developed further after the project was presented.

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
- PostgreSQL: You can download and install PostgreSQL from [here](https://www.postgresql.org/download/).
- NGINX: You can download and install NGINX from [here](https://nginx.org/en/download.html).

## Getting Started
To get started with the project, follow these steps:
``` shell
# Clone the repository:
   git clone https://github.com/AnythingAnywhereEverything/Chromatic

# Navigate to the project directory:
   cd chromatic

# Please configure your environment variables in the .env file before running the application. You can copy the .env.example file and rename it to .env, then fill in the necessary values.

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
Access the application by navigating to `http://localhost` in your web browser. You should see the frontend interface, and you can interact with the backend services implemented in Rust, and Elixir.

## Contributing
Contributions to this project are welcome! If you have any improvements or new features to add, please feel free to submit a pull request. Make sure to follow the existing code style and include tests for any new functionality.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Project Owners

<p align="left">
  <a href="https://github.com/ZartexVertagen">
    <img src="https://github.com/ZartexVertagen.png" width="64px" style="border-radius: 50%;" align="left" alt="ZartexVertagen"/>
  </a>
  &nbsp; <a href="https://github.com/ZartexVertagen"><b>@ZartexVertagen</b></a>
  <br />&nbsp; Leading Developer and Project Owner
</p>
<br clear="left"/>

<p align="left">
  <a href="https://github.com/K-Hongkaew">
    <img src="https://github.com/K-Hongkaew.png" width="64px" style="border-radius: 50%;" align="left" alt="K-Hongkaew"/>
  </a>
  &nbsp; <a href="https://github.com/K-Hongkaew"><b>@K-Hongkaew</b></a>
  <br />&nbsp; Co-Developer and Project Owner
</p>
<br clear="left"/>
