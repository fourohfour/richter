import requests

response = requests.get("https://api.showmyhomework.co.uk/api/calendars?classGroup=7A%2FGg1",
                                headers = {"Accept" : "application/smhw.v3+json"})

print(response.text)
