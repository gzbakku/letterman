from commands import Commands
import executer
import creds

def init():

    w = executer.confirm()
    if not w:
        return

    c = creds.init_creds("primary",w)
    s = creds.Session(c)

    q = input("press y to continue")
    if q != "y":
        return

    secondary_username = c["secondary_username"]
    secondary_password = c["secondary_password"]

    if False:
        session = executer.start_session(s)
        builder = Commands()
        builder.log("adding user")
        builder.command("sudo -k adduser " + secondary_username)
        builder.reply("password",secondary_password)
        builder.reply("password",secondary_password)
        builder.print()
        builder.execute(session,s)

    if False:
        session = executer.start_session(s)
        builder = Commands()
        builder.log("adding user password")
        builder.command("sudo -k passwd " + secondary_username)
        builder.reply("password",secondary_password)
        builder.reply("password",secondary_password)
        builder.reply("password",secondary_password)
        builder.print()
        builder.execute(session,s)

    if True:
        session = executer.start_session(s)
        builder = Commands()
        builder.log("adding user to sudo group")
        builder.command("usermod -aG sudo " + secondary_username)
        # builder.reply("password",secondary_password)
        builder.print()
        builder.execute(session,s)    

    if False:
        session = executer.start_session(s)
        builder = Commands()

        builder.log("removing existing home dir")
        builder.command("cd /home/;sudo -k rm -rf " + secondary_username)
        builder.reply("password",secondary_password)
        builder.print()

        builder.log("adding new home dir")
        builder.command("cd /home/;sudo -k mkdir " + secondary_username)
        builder.reply("password",secondary_password)
        builder.print()

        builder.log("adding permissions to home dir")
        builder.command("cd /home/;sudo -k chown " + secondary_username + " " + secondary_username)
        builder.reply("password",secondary_password)
        builder.print()

        builder.execute(session,s)    

init()



