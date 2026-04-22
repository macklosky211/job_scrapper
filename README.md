# My job scrapper

This was just a simple tool to see which of my favorite companies had new open positions, 
but overall this project just takes a current snapshot of each site and compares them everytime the program runs.

---
# Technical Overview

It just pulls the raw body using the ureq library and hashes it, then compares this hash with the last hash generated for the site.
Any hashes that dont match means the website changed, likely indicating a new job posting if you put a valid url.

---
# Usage

1. clone this repo.

2. create 'websites.csv' in the root folder

3. Add any url's (seperated by new lines)
> example: 
  ```
  https://myexample.com/jobs
  https://anotherjobsite.com/more-jobs
  ```

4. run 'cargo run'
    * if you want to, you can just do 'cargo build --release' and run the appropriate file instead

---
