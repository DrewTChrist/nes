from comm import Communicator
from ui import UI 

import queue
import sys
import tkinter as tk
import threading
import json
from json.decoder import JSONDecodeError

HEIGHT = 500
WIDTH = 1000
POLL_TIME_MS = 100

class App:
    def __init__(self, master):
        self.master = master
        self.master.geometry(f"{WIDTH}x{HEIGHT}")
        self.master.maxsize(WIDTH, HEIGHT)
        self.master.minsize(WIDTH, HEIGHT)
        self.master.title("Debugger")

        self.comm = Communicator()

        self.queue = queue.Queue()

        self.gui = UI(master, self.queue, self.comm.list_ports(), self.quit)

        self.running = True
        self.comm_thread = threading.Thread(target=self.read_comms, daemon=True)
        self.comm_thread.start()

        self.poll_ui()

    """ 
    Regularly calls the handle_message method
    for the UI and checks if the application
    is ready to close
    """
    def poll_ui(self):
        self.gui.handle_message()
        if not self.running:
            self.comm.serial.close()
            sys.exit(1)
        """ Too small of a time will lock up the UI
            I will need to find the sweet spot """
        self.master.after(POLL_TIME_MS, self.poll_ui)

    """
    This is the thread target that reads
    serial data from the micro controller
    and stores it in the message queue for
    the UI class
    """
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
                    data = self.comm.readline()
                    try:
                        msg = json.loads(data)
                        self.queue.put(msg)
                    except JSONDecodeError:
                        data = self.comm.readline()
                        msg = json.loads(data)
                        self.queue.put(msg)

    """
    Quits the application when called
    """
    def quit(self):
        self.running = False
