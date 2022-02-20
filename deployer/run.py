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
        
        executer.write(session,"sudo -k netstat -tulpn")
        executer.reply(session,"password",secondary_password)

        connections = executer.read(session).split("\n")
        netstat_regex = re.compile(r'([\w]+)\s+([\d]+)\s+([\d]+)\s+(([\d\w.]+):([\d*]+)+)\s+(([\d\w:.]+):([\d*]+)+)\s+([\d\w]+)\s+(([\d]+)\/([\w\d\s\W]+))')

        processes = []
        for line in connections:

            search = netstat_regex.search(line)
            if search:
                pid = search.group(12)
                external_port = search.group(6)
                if external_port == '80':
                    processes.append(pid)
                if external_port == '443':
                    processes.append(pid)
                if external_port == '587':
                    processes.append(pid)
                if external_port == '2525':
                    processes.append(pid)                    

        print("killing connection holding processes")
        print(processes)

        session = executer.start_session(s)

        for pid in processes:
            session = executer.start_session(s)
            print("killing pid : {pid}".format(pid=pid))
            executer.write(session,"sudo -k kill -9 {pid}".format(pid=pid))
            executer.reply(session,"password",secondary_password)
            e = executer.read(session)
            # print(e)

    if True:

        session = executer.start_session(s)
        
        executer.write(session,"sudo -k ps -a")
        executer.reply(session,"password",secondary_password)

        instances = executer.read(session).split("\n")
        ps_regex = re.compile(r'([\d]+)\s([\w\d\/]+)\s+([\d:]+)\s([\w\W]+)')

        processes = []
        for line in instances:
            search = ps_regex.search(line)
            if search:
                process = search.group(4)
                if "mailcenter" in process and "defunct" not in process:
                    pid = search.group(1)
                    processes.append(pid)

        print("killing mailcenter processes")
        print(processes)              

        for pid in processes:
            print("killing pid : {pid}".format(pid=pid))
            session = executer.start_session(s)
            executer.write(session,"sudo -k kill -9 {pid}".format(pid=pid))
            executer.reply(session,"password",secondary_password)
            e = executer.read(session)
            # print(e)         

    if True:

        session = executer.start_session(s)

        # sudo -k ./mailcenter &>> mailcenter_log.txt  

        executer.write(
            session,
            "cd /home/{secondary_username}/mailcenter;"
            .format(
                secondary_username=secondary_username
            ) +
            # "sudo -k ./mailcenter",
            "sudo -k ./mailcenter &>> mailcenter_log.txt"
        )

        print("executing mailcenter")

        executer.reply(session,"password",secondary_password)

        print("password sent")

        # executer.print_infinite(session)

        e = executer.read_limit(session,10)

        print(e)


main()    