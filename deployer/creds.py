import json
import sys

# global username = ""
# password = ""
# port = 22
# ip = ""

login_with = ""

def init_creds(t,w):

    c_path = "d://workstation/expo/rust/letterman/secret/server_creds.json"

    if "-u" in sys.argv:
        c_path = "/mnt/d/workstation/expo/rust/letterman/secret/server_creds.json"

    f = open(c_path)
    creds = json.load(f)

    # w = "local"

    print("w {w} t {t} host".format(w=w,t=t))
    

    username = ''
    password = ''
    port = creds[w]["port"]
    ip = creds[w]["ip"]
    primary_username = creds[w]["primary"]["username"]
    primary_password = creds[w]["primary"]["password"]
    secondary_username = creds[w]["secondary"]["username"]
    secondary_password = creds[w]["secondary"]["password"]

    

    if t == "primary":
        username = primary_username
        password = primary_password
    if t == "secondary":
        username = secondary_username
        password = secondary_password

    print("port {port}".format(port=port))
    print("host {host}".format(host=ip))
    print("username {username}".format(username=username))
    # print("password {password}".format(password=password))

    return {
        "username":username,
        "password":password,
        "port":port,
        "host":ip,
        "primary_username":primary_username,
        "primary_password":primary_password,
        "secondary_username":secondary_username,
        "secondary_password":secondary_password,
    }

class Session:

    def __init__(self,s):
        self.username = s["username"]
        self.password = s["password"]
        self.port = s["port"]
        self.host = s["host"]