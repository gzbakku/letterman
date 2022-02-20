import paramiko
import sys

def start_upload_transport(s):
    transport = paramiko.Transport((s.host, s.port))
    transport.connect(username = s.username, password = s.password)
    return paramiko.SFTPClient.from_transport(transport)

def start_client(s):
    client = paramiko.client.SSHClient()
    client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
    client.connect(s.host, username=s.username, password=s.password)
    return client

def start_session(s):
    client = start_client(s)
    transport = client.get_transport()
    session = transport.open_session()
    session.set_combine_stderr(True)
    session.get_pty()
    session.setblocking(1)
    return session

def log(v):
    print(">>> " + v)

def confirm():
    if "--server" in sys.argv:
        return "server"
    if "--local"  in sys.argv:
        return "local"
    ask = input("s for server l for local ")
    if ask == "l":
        return "local"
    if ask == "s":
        return "server"
    return False        

def write(session,m):
    session.exec_command(m)

def read(session):
    collect = ''
    while session.recv_ready()==False:
        stdout=session.recv(512)
        if len(stdout) == 0:
            break
        else:
            collect += stdout.decode("utf-8")
    return collect     

def read_limit(session,l):
    collect = ''
    while session.recv_ready()==False and len(collect) < l:
        stdout=session.recv(512)
        if len(stdout) == 0:
            break
        else:
            collect += stdout.decode("utf-8")
    return collect    

def print_infinite(session):
    while session.recv_ready()==False:
        stdout=session.recv(512)
        if len(stdout) > 0:
            print(stdout.decode("utf-8"))
    print("print_infinite ended")

def reply(session,w,r):
    collect = ''
    while session.recv_ready()==False:
        stdout=session.recv(512)
        if len(stdout) == 0:
            break
        else:
            collect += stdout.decode("utf-8")
            if w in collect:
                session.send(r+'\n')
                return True
    if w in collect:
        session.send(r+'\n')
        return True
    else:
        return False