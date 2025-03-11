# Web_server
Simple web server using classical thread pooling from my personally made thread pooling library.

### Functionalities

- Serve two http pages localted at `web_server/www/` folders for the moment.
- Multi-threaded servicing.
- Thread-pooling library.

### To add functionalities

- Dynamic page serving.
- More integration with user environment.
- Upgrade CLI.
- Load balancing across different servers.
- Merging to async.
- Caching.

ps: Project is still under development, and lot of functionalities has to be added.

## How to use

1. Clone the repo in your home directory:

  ```bash
  cd $home
  git clone https://github.com/cunbex/web_server.git
  ```

2. Update the .env file:

  ```plaintext

  example:

  www_path = "/home/username/web_server/www/"
  thread_pool_size = "4"

  ```

3. Launch the binary

  ```bash
  ./web_server
  ```

4. Visit website:

  localhost:7878 -> default hello.html page.
  localhost:7878/sleep -> 5 sec sleep page to demonstrate multi-threaded serving.
