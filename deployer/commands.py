import executer

class Commands:

    def __init__(self):
        self.commands = []

    def log(self,m):
        self.commands.append({
            "t":"l",
            "m":m,
        })
        
    def command(self,c):
        self.commands.append({
            "t":"e",
            "c":c,
        })

    def reply(self,w,r):
        self.commands.append({
            "t":"rp",
            "w":w,
            "r":r,
        })

    def read(self):
        self.commands.append({
            "t":"re"
        })

    def print(self):
        self.commands.append({
            "t":"pr"
        })        

    def execute(self,session,s):
        for i in self.commands:
            if i["t"] == "e":
                try:
                    session.exec_command(i["c"])
                except:
                    session = executer.start_session(s)
                    try:
                        session.exec_command(i["c"])
                    except:
                        break
            elif i["t"] == "rp":
                hold = executer.reply(session,i["w"],i["r"])
                print("reply ",hold)
            elif i["t"] == "re":
                executer.read(session)
            elif i["t"] == "pr":
                print(executer.read(session))      
            elif i["t"] == "l":
                print(">>> " + i["m"])            

