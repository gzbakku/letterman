
import executer
from commands import *
from uploader import Uploader
import creds

def main():

    c = creds.init_creds("secondary","server")
    s = creds.Session(c)

    q = input("press y to continue")
    if q != "y":
        return

    secondary_password = c["secondary_password"]
    secondary_username = c["secondary_username"]

    if True:
        session = executer.start_session(s)
        builder = Commands()

        # ---------------------------------
        # change letsencrypt live permissions
        # ---------------------------------
        builder.log("removing mailcenter dir")
        builder.command(
            "cd /etc/letsencrypt;sudo -k chown {secondary_username} live"
            .format(
                secondary_username=secondary_username
            )
        )
        builder.reply("password",secondary_password)
        builder.print()

        # ---------------------------------
        # smtp ssl
        # ---------------------------------

        # ---------------------------------
        # add smtp cert
        # ---------------------------------
        builder.log("copying smtp_gzbemail_xyz cert")
        builder.command(
            "sudo -k cp /etc/letsencrypt/live/smtp.gzbemail.xyz/fullchain.pem " +
            "/home/{secondary_username}/mailcenter/smtp_gzbemail_xyz.cert"
            .format(
                secondary_username=secondary_username
            )
        )
        builder.reply("password",secondary_password)
        builder.print()

        builder.log("permitting smtp_gzbemail_xyz cert")
        builder.command(
            "cd /home/{secondary_username}/mailcenter;sudo -k chown akku smtp_gzbemail_xyz.cert"
            .format(secondary_username=secondary_username)
        )
        builder.reply("password",secondary_password)
        builder.print()

        # ---------------------------------
        # add smtp key
        # ---------------------------------
        builder.log("copying smtp.gzbemail.xyz private_key")
        builder.command(
            "sudo -k cp /etc/letsencrypt/live/smtp.gzbemail.xyz/privkey.pem " +
            "/home/{secondary_username}/mailcenter/smtp_gzbemail_xyz.private_key"
            .format(
                secondary_username=secondary_username
            )
        )
        builder.reply("password",secondary_password)
        builder.print()

        builder.log("permitting smtp_gzbemail_xyz private_key")
        builder.command(
            "cd /home/{secondary_username}/mailcenter;sudo -k chown akku smtp_gzbemail_xyz.private_key"
            .format(secondary_username=secondary_username)
        )
        builder.reply("password",secondary_password)
        builder.print()

        # ---------------------------------
        # api ssl
        # ---------------------------------

        # ---------------------------------
        # add api cert
        # ---------------------------------
        builder.log("copying api_gzbemail_xyz cert")
        builder.command(
            "sudo -k cp /etc/letsencrypt/live/api.gzbemail.xyz/fullchain.pem " +
            "/home/{secondary_username}/mailcenter/api_gzbemail_xyz.cert"
            .format(
                secondary_username=secondary_username
            )
        )
        builder.reply("password",secondary_password)
        builder.print()

        builder.log("permitting api_gzbemail_xyz cert")
        builder.command(
            "cd /home/{secondary_username}/mailcenter;sudo -k chown akku api_gzbemail_xyz.cert"
            .format(secondary_username=secondary_username)
        )
        builder.reply("password",secondary_password)
        builder.print()

        # ---------------------------------
        # add api key
        # ---------------------------------
        builder.log("copying api.gzbemail.xyz key")
        builder.command(
            "sudo -k cp /etc/letsencrypt/live/api.gzbemail.xyz/privkey.pem " +
            "/home/{secondary_username}/mailcenter/api_gzbemail_xyz.private_key"
            .format(
                secondary_username=secondary_username
            )
        )
        builder.reply("password",secondary_password)
        builder.print()

        builder.log("permitting api.gzbemail.xyz private_key")
        builder.command(
            "cd /home/{secondary_username}/mailcenter;sudo -k chown akku api_gzbemail_xyz.private_key"
            .format(secondary_username=secondary_username)
        )
        builder.reply("password",secondary_password)
        builder.print()
        

        builder.execute(session,s)

main()    