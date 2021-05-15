# book-db

Small sample-app using [Rocket](https://rocket.rs/).

## How to run

    cd backend
    docker build -t book-db .
    docker run -it --publish 8000:8000 book-db
