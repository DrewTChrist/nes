from comm import Communicator
from ui import UI 

import queue
import sys
import tkinter as tk
import threading
import time

HEIGHT = 500
WIDTH = 1000

class App:
    def __init__(self, master):
        self.master = master
        self.master.geometry(f"{WIDTH}x{HEIGHT}")
        self.master.maxsize(WIDTH, HEIGHT)
        self.master.minsize(WIDTH, HEIGHT)
        self.master.title("Debugger")
        self.comm = Communicator()

        self.queue = queue.Queue()

        self.gui = UI(master, self.queue, self.comm.list_ports(), self.endApplication)

        self.running = True
        self.comm_thread = threading.Thread(target=self.read_comms, daemon=True)
        self.comm_thread.start()

        self.poll_ui()

    def poll_ui(self):
        self.gui.handle_message()
        if not self.running:
            self.comm.serial.close()
            sys.exit(1)
        self.master.after(20, self.poll_ui)

    def read_comms(self):
        while self.running:
            if self.gui.started:
                port = self.gui.select_port()
                if port is None:
                    self.gui.started = False
                else:
                    self.comm.set_port(self.gui.select_port())
                    if not self.comm.serial.is_open:
                        self.comm.open()
                    msg = self.comm.readline()
                    self.queue.put(msg)

    def endApplication(self):
        self.running = False
