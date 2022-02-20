import executer
from commands import *
from uploader import Uploader
import creds
import sys

def main():

    w = executer.confirm()
    if not w:
        return

    c = creds.init_creds("secondary",w)
    s = creds.Session(c)

    secondary_password = c["secondary_password"]
    secondary_username = c["secondary_username"]

    if True:

        session = executer.start_session(s)

        executer.log("logging mailcenter dir")

        executer.write(session,"sudo -k tail -f /home/{secondary_username}/mailcenter/mailcenter_log.txt".format(secondary_username=secondary_username))

        #  sudo -k tail -f /home/akku/mailcenter/mailcenter_log.txt

        executer.reply(session,"password",secondary_password)

        executer.print_infinite(session)

main()