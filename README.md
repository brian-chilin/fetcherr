# unfinished. currently only builds rust programs

# fetcherr
Niche and unsupported program only intended to run on Debian. Intended use is to await a POST request (likely sent from GitHub workflow) then fetch a project, compile, stop the running instance, and restart the new&freshly compiled program
#### Use case
Websites that are incrementally updated quickly. Setting up fetcherr would be a waste of time for sites that only need to fetch from version control & build once in a while. Setting up fetcherr will make it very easy to keep my portfolio updated. I can make a variable amount of commits, then send a single signal to tell a server to re-deploy. Fetcherr's aim is to take away all the steps after I make the POST request, and to seamlessly handle re-deployment. This is notably easy in conjunction with supervisor.

# instructions TODO use https
### Pre-reqs
Githup repo where the top level is an IntelliJ Rust(plugin) program.
The machine hosting must have paths properly set up, so typing 'cargo' in any directory should show help for Rust's package manager.

Will use 'supervisor' available at http://supervisord.org/index.html
### Config
Make secrets.json by copying secretsFORMAT.json and fill in url & key with something secret you choose. I'm no expert but my gut tells me to recommend UUIDs. 
Also fill in the IP with IP of the machine running fetcherr. This is often in a format like 192.168.1.X
Also, fill in a port to listen on. I'll be using 9000 for this guide

proj_cargo_toml should be the Cargo.toml file of your project. The only conventions I've tested are C:\\path\\to\\Cargo.toml on Windows10
and /path/to/Cargo.toml on Debian11 and they seem to work. 
### Post Request Setup
You will make GitHub make a POST request to http://ip:9000/SECRETS_URL with a JSON header "key: SECRETS_KEY"

Example using curl: curl -i -X POST http://ip:9000/my_url -H "key: my_key"

Generate a token at https://github.com/settings/tokens for the value of "gh_token"

Have GitHub workflow/event make a request to the url with the key as the header. Recommended to use uuid's as url/key. Returns 404 on other url's
