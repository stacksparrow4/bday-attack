import requests

resp = requests.get("https://media.openlearning.com/BQmL2ravMff6UdksLapzTdfvV4q6BDwPY4J9u3Gc7DiEe8KK64yqdvksgwdRm4ii.1616372982/confession_real.txt")
with open("src/confession_real.txt", "wb") as f:
    f.write(resp.content)

resp = requests.get("https://media.openlearning.com/A7vUMFpMTKP9DPBgemhTiguECfdZVqxW3V5z6KmiLwVTSTbCc36knN8kEQXmvXAP.1616373012/confession_fake.txt")
with open("src/confession_fake.txt", "wb") as f:
    f.write(resp.content)
