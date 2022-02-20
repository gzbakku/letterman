import paramiko
from executer import start_upload_transport 

class Uploader:

    def __init__(self,s):
        self.sftp = start_upload_transport(s)

    def test(self):
        print(self)

    def log(self,m):
        print(">>> " + m)

    def upload(self,local_path,server_path):
        self.sftp.put(local_path,server_path)

    def end(self):
        self.sftp.close()
        self.client.close()
