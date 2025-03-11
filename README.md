# Web_server
Simple web server with personal thread pooling library.

## Structure

- src/lib.rs: Thread-pooling lbirary.
- src/main.rs: Web serving binary.

### Features

- Serve two HTTP pages located in the `web_server/www/` folder for now.  
- Multi-threaded request handling.  
- Thread-pooling library.  

### Planned Features  

- Dynamic page serving.  
- Better integration with the user environment.  
- CLI improvements.  
- Load balancing across multiple servers.  
- Migration to an async model.  
- Caching.  

**Note:** This project is still under development. It started as an educational project and will be expanded with many more features.  

## How to Use  

1. Clone the repository into your home directory:  
```bash
   cd $HOME
   git clone https://github.com/cunbex/web_server.git
````

2. Update the `.env` file:
```plaintext
  Example:

  www_path = "/home/username/web_server/www/"
  thread_pool_size = "4"

```
3. Launch the binary:
```bash
  ./web_server

```
4. Open the website:
  - Visit localhost:7878 → Default hello.html page.
  - Visit localhost:7878/sleep → A 5-second sleep page to demonstrate multi-threaded request handling.
