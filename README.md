### INSECURE TO USE OVER OPEN INTERNET
#### unfinished. currently only builds a single RUST program at a time.
# fetcherr
Niche and unsupported program only intended to run on Debian.
Intended use is to await a POST request (likely sent from GitHub workflow) then fetch a project, compile, stop the running instance, and restart the new & freshly compiled program
#### Use case
Websites that are incrementally updated quickly.
Setting up fetcherr would be a waste of time for sites that only need to fetch from version control & build once in a while.
Setting up fetcherr will make it very easy to keep my portfolio updated.
I can make a variable amount of commits, then send a single signal to tell a server to re-deploy.
Fetcherr's aim is to take away all the steps after I make the POST request, and to seamlessly handle re-deployment.
This is notably easy in conjunction with a process manager like systemctl or supervisord

# instructions
### Pre-reqs
* Fetcherr is **NOT SECURE** by any means. Post requests are done over open http,
 and I could only recommend using fetcherr over open internet (like GH workflows curl requests) if you have nginx sit in front of fetcherr or another means of using https.
 I've already configured my nginx as a reverse proxy that handles ssl for my website,
 so it is easy for me to add another server in my nginx config for fetcherr.
 Because this is my use case, I've decided to not attempt to implement ssl myself into fetcherr. 
* Fetcherr only works for Githup repos where the top level is an IntelliJ Rust(plugin) program.
* **make sure you use release build of your program. executable should be in target/release. details of release build here**

  https://doc.rust-lang.org/book/ch14-01-release-profiles.html
* The host machine needs systemctl.
 Please have a working and enabled systemctl unit for the program you intend to fetch, build, and restart. 
* The host machine needs rustup and the stable toolchain for the system.
 After rustup is installed the proper toolchain can be installed with 'rustup toolchain install stable'


### Config
1. Make **config.json** by copying configFORMAT.json in the top level directory and fill in **url & key** with something secret you choose.
I'm no expert but my gut tells me to recommend UUIDs. 
Also fill in the **IP** with the IP of the machine running fetcherr. This is often in a format like 192.168.1.X
Also, fill in a **port** to listen on. I'll be using 9000 for this guide 

2. **proj_cargo_toml** should be the Cargo.toml file of your project.
The only convention I've tested is /path/to/Cargo.toml on Debian11

3. **rustup_home** should be the directory rustup itself is installed.
For me, this is "/home/brian/.rustup"
and I had followed directions on https://rustup.rs/ leaving defaults.
It's imperative that rustup_home/toolchains contains the stable toolchain.
For me this is 'stable-x86_64-unknown-linux-gnu'.
If you take a look inside this toolchain folder it's important to find
the bin folder with executables like cargo. 

### Post Request Setup
You will make GitHub make a POST request to http://ip:9000/SECRETS_URL with a JSON header "key: SECRETS_KEY".
I've never actually used GH workflow, but my best idea is to trigger a curl request from push.
This would work particularly nice if you could trigger only after reading the descriptions of commits and looking for a keyword. 

Example using curl: curl -i -X POST http://ip:9000/my_url -H "key: my_key"

**Another alternative to using GH workflows would be to just leave this as a script somewhere that is quick to execute.**

Generate a token at https://github.com/settings/tokens for the value of "gh_token"

Have GitHub workflow/event make a request to the url with the key as the header. Returns 404 on other url's
