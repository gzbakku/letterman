import executer
from commands import *
from uploader import Uploader
import creds
import sys

def main():

    print("king ping")

    w = executer.confirm()
    if not w:
        return

    c = creds.init_creds("secondary",w)
    s = creds.Session(c)

    secondary_password = c["secondary_password"]
    secondary_username = c["secondary_username"]

    print("secondary_username : " + secondary_username)

    if True:
        session = executer.start_session(s)
        builder = Commands()

        builder.log("removing letterman executable")
        builder.command("cd /home/{secondary_username}/;sudo -k rm -rf letterman".format(secondary_username=secondary_username))
        builder.reply("password",secondary_password)
        builder.print()

        builder.execute(session,s)

    if True:

        client = Uploader(s)

        base_path = "D:/workstation/expo/rust/letterman"
        if "-u" in sys.argv:
            base_path = "/mnt/d/workstation/expo/rust/letterman"

        print(base_path +"/secret/ge_dkim_private_key.txt")

        client.log("uploading letterman 1 executable")
        client.upload(
            base_path +"/letterman/target/release/letterman",
            "letterman"
        )

        print(base_path +"/secret/ge_dkim_private_key.txt")

        client.log("uploading ge_private_key executable")
        client.upload(
            base_path +"/secret/ge_dkim_private_key.txt",
            "ge_dkim_private_key.txt"
        )

    if True:
        session = executer.start_session(s)
        builder = Commands()

        builder.log("making letterman executable")
        builder.command("cd /home/{secondary_username}/;sudo -k chmod +x letterman".format(secondary_username=secondary_username))
        builder.reply("password",secondary_password)
        builder.print()

        builder.log("listing letterman stat")
        builder.command("cd /home/{secondary_username}/;ls;stat letterman".format(secondary_username=secondary_username))
        builder.print()

        builder.execute(session,s)

main()    

# export MAILCENTER_CONFIG=/home/akku/mailcenter/config.json; echo mailcenter_config=$MAILCENTER_CONFIG