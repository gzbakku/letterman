import re
import executer
from commands import *
from uploader import Uploader
import creds

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

        # sudo -k ./mailcenter &>> mailcenter_log.txt  

        executer.write(
            session,
            "cd /home/{secondary_username}/;"
            .format(
                secondary_username=secondary_username
            ) +
            # "sudo -k ./mailcenter",
            "sudo -k ./letterman &>> letterman_log.txt"
        )

        print("executing letterman")

        executer.reply(session,"password",secondary_password)

        print("password sent")

        # executer.print_infinite(session)

        e = executer.read_limit(session,10)

        print(e)


main()    