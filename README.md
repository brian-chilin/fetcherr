# fetcherr
Rusty GitHub Based on GH Workflow/Events

# instruction
Make secrets.json by copying secretsFORMAT.json & fill in url and key

Have GitHub workflow/event make a request to the url with the key as the header. Recommended to use uuid's as url/key. Returns 404 on other url's

Where to make request http://ip:8080/url