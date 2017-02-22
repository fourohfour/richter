import requests

response = requests.get("https://api.showmyhomework.co.uk/api/schools?subdomain=stanborough",
                                headers = {"Accept" : "application/smh.v3+json"})

print(response.text)
